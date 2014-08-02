// This attribute specifies that the Rust compiler will output a dynamic
// library when this file is compiled.
// Generally, many Rust libraries will *also* have a `#![crate_type = "rlib"]`
// attribute set, which means the Rust compiler will produce a static library.
// However, libraries which provide syntax extensions must be dynamically
// linked with `libsyntax`, so we elide the `rlib` and only produce a dynamic
// library.
#![crate_type = "dylib"]
 
// Enable the `plugin_registrar` feature (which is the compiler hook).
#![feature(plugin_registrar, quote)]
 
extern crate rustc;
extern crate syntax;
 
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

fn five_expand(cx: &mut ExtCtxt, _: codemap::Span, _: &[ast::TokenTree]) -> Box<MacResult> {
    let n: uint = 5;
    MacExpr::new(quote_expr!(cx, $n))
}
