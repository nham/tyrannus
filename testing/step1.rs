#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use syntax::ast::{mod, Item, MutImmutable, Inherited, Ident};
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr};
use syntax::parse::token::intern;

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

fn expand(cx: &mut ExtCtxt, sp: codemap::Span, _: &[ast::TokenTree]) -> Box<MacResult> {
    let mut v = vec!();
    v.push( quote_item!(cx, 
        fn foo(n: uint) -> (&'static str, &'static str) {
            (foo_str.slice_to(n), foo_str.slice_from(n))
        }
    ).unwrap() );

    let item = box(GC) ast::Item {
        ident: Ident::new(intern("foo_str")),
        attrs: ,
        id: ast::DUMMY_NODE_ID,
        node: ast::ItemStatic(_, MutImmutable, _),
        vis: Inherited,
        span: sp,
    };

    //v.push( quote_item!(cx, static foo_str: &'static str = "abc";).unwrap() );

    box MacItems { items: v } as Box<MacResult>
}



/*
    fn item(&self, span: Span,
            name: Ident, attrs: Vec<ast::Attribute>,
            node: ast::Item_) -> Gc<ast::Item> {
        // FIXME: Would be nice if our generated code didn't violate
        // Rust coding conventions
        box(GC) ast::Item { ident: name,
                    attrs: attrs,
                    id: ast::DUMMY_NODE_ID,
                    node: node,
                    vis: ast::Inherited,
                    span: span }
    }
 */

/*
    fn item_static(&self,
                   span: Span,
                   name: Ident,
                   ty: P<ast::Ty>,
                   mutbl: ast::Mutability,
                   expr: Gc<ast::Expr>)
                   -> Gc<ast::Item> {
        self.item(span, name, Vec::new(), ast::ItemStatic(ty, mutbl, expr))
    }
 */
