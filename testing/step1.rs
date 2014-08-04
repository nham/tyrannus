#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use syntax::ast::{mod};
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, MacItem};

use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("foo", expand);
}

fn expand(cx: &mut ExtCtxt, _: codemap::Span, _: &[ast::TokenTree]) -> Box<MacResult> {
    MacItem::new( quote_item!(cx, static foo_str: &'static str = "abc";).unwrap() )
}
