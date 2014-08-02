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
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr, DummyResult};
use syntax::parse::{mod, token};
use rustc::plugin::Registry;
 
// register_macro takes a name (&str) and a MacroExpanderFn
// type MacroExpanderFn = fn(ecx: &mut ExtCtxt, span: Span, token_tree: &[TokenTree]) -> Box<MacResult>;
// register_macro "is a convenience wrapper for register_syntax_extension. It builds for you a NormalTT with a BasicMacroExpander, and also takes care of interning the macro's name."
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("add_five", add_five_expand)
}

fn add_five_expand(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let x = match parse(cx, tts) {
        Some(n) => n,
        None => return DummyResult::expr(sp),
    };
    let n: uint = x + 5;
    MacExpr::new(quote_expr!(cx, $n))
}

fn parse(cx: &mut ExtCtxt, tts: &[ast::TokenTree]) -> Option<uint> {
    use syntax::print::pprust;

    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(),
                                                Vec::from_slice(tts));

    let arg = parser.parse_expr();
    match arg.node {
        ast::ExprLit(spanned) => {
            match spanned.node {
                ast::LitUint(n, _) => {
                    if !parser.eat(&token::EOF) {
                        cx.span_err(parser.span,
                                    "expected only one integer literal");
                        return None

                    }
                    return Some(n as uint)
                },
                _ => {}
            }
        },
        _ => {}
    }

    let err = format!("expected unsigned integer literal but got `{}`", pprust::expr_to_string(&*arg));
    cx.span_err(parser.span, err.as_slice());
    None
}
