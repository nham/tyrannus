#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use syntax::ast::{mod, Item, MutImmutable, Inherited, Ident};
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use syntax::parse::token;
use syntax::print::pprust;

use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("parse_str", expand);
}

struct MacItems {
    items: Vec<::std::gc::Gc<Item>>,
}

impl MacResult for MacItems {
    fn make_def(&self) -> Option<::syntax::ext::base::MacroDef> { None }
    fn make_expr(&self) -> Option<::std::gc::Gc<ast::Expr>> { None }
    fn make_pat(&self) -> Option<::std::gc::Gc<ast::Pat>> { None }
    fn make_stmt(&self) -> Option<::std::gc::Gc<ast::Stmt>> { None }

    fn make_items(&self) -> Option<::syntax::util::small_vector::SmallVector<::std::gc::Gc<Item>>> {
        Some(::syntax::util::small_vector::SmallVector::many(self.items.clone()))
    }
}

fn expand(mut cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let x = match parse_string(cx, tts) {
        Some(s) => s,
        None => return DummyResult::any(sp), // TODO: why any?
    };
    let y = x.as_slice();

    let mut v = vec!();

    v.push( quote_item!(&mut cx,
        fn foo(n: uint) -> (&'static str, &'static str) {
            (foo_str.slice_to(n), foo_str.slice_from(n))
        }
    ).unwrap() );
    v.push( quote_item!(cx, static foo_str: &'static str = $y;).unwrap() );

    box MacItems { items: v } as Box<MacResult>
}

fn parse_string(cx: &mut ExtCtxt, tts: &[ast::TokenTree]) -> Option<String> {
    let mut parser = cx.new_parser_from_tts(tts);
    let arg = cx.expand_expr(parser.parse_expr());

    let s = match arg.node {
        ast::ExprLit(lit) => {
            match lit.node {
                ast::LitStr(ref s, _) => s.to_string(),
                _ => {
                    cx.span_err(arg.span, format!(
                        "expected string literal but got `{}`",
                        pprust::lit_to_string(&*lit)).as_slice());
                    return None
                }
            }
        },
        _ => {
            cx.span_err(arg.span, format!(
                "expected string literal but got `{}`",
                pprust::expr_to_string(&*arg)).as_slice());
            return None
        }
    };

    if !parser.eat(&token::EOF) {
        cx.span_err(parser.span, "only one string literal allowed");
        return None;
    }
    Some(s)
}
