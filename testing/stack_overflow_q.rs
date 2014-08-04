#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use syntax::ast::{mod, Item};
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult};

use rustc::plugin::Registry;

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

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("foo", expand);
}

fn expand(cx: &mut ExtCtxt, sp: codemap::Span, _: &[ast::TokenTree]) -> Box<MacResult> {
    let mut v = vec!();

    v.push( quote_item!(cx, static x: uint = 5;).unwrap() );
    v.push( quote_item!(cx, fn bar() -> uint { x + 1 }).unwrap() );

    box MacItems { items: v } as Box<MacResult>
}
