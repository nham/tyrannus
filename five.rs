// This attribute specifies that the Rust compiler will output a dynamic
// library when this file is compiled.
// Generally, many Rust libraries will *also* have a `#![crate_type = "rlib"]`
// attribute set, which means the Rust compiler will produce a static library.
// However, libraries which provide syntax extensions must be dynamically
// linked with `libsyntax`, so we elide the `rlib` and only produce a dynamic
// library.
#![crate_type = "dylib"]
 
// Enable the `plugin_registrar` feature (which is the compiler hook).
#![feature(plugin_registrar)]
 
extern crate rustc;
extern crate syntax;
 
use std::gc::GC;
use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr};
use rustc::plugin::Registry;
 
// register_macro takes a name (&str) and a MacroExpanderFn
// type MacroExpanderFn = fn(ecx: &mut ExtCtxt, span: Span, token_tree: &[TokenTree]) -> Box<MacResult>;
// register_macro "is a convenience wrapper for register_syntax_extension. It builds for you a NormalTT with a BasicMacroExpander, and also takes care of interning the macro's name."
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("five", five_expand)
}

// fn respan<T>(sp: Span, t: T) -> Spanned<T>
//
// pub struct Expr {
//   pub id: NodeId,
//   pub node: Expr_,
//   pub span: Span
// }
//
// the Expr_ we're using is the ExprLit, which is an enum variant taking
// a Gc<Lit> = Gc<Spanned<Lit_>>
//  
//  pub struct Spanned<T> {
//    pub node: T,
//    pub span: Span,
//  }
//
// "Spans represent a region of code, used for error reporting.
// Positions in spans are *absolute* positions from beginning of the
// codemap, not positions relative to FileMaps."
//
// if you look, a CodeMap is just a struct with one field whose type is essentially
// a vector of FileMaps (actually RefCell<Vector<Rc<FileMap>>>), so mathematically
// a sequence of FileMaps
 
fn five_expand(_: &mut ExtCtxt, sp: codemap::Span, _: &[ast::TokenTree]) -> Box<MacResult> {
    let n: u64 = 5;
    let lit = ast::LitUint(n, ast::TyU); // this should be of type Lit_
    let spanned = box(GC) codemap::respan(sp, lit);

    let expr = box(GC) ast::Expr {
        id: ast::DUMMY_NODE_ID,
        node: ast::ExprLit(spanned),
        span: sp,
    };

    MacExpr::new(expr)
}
