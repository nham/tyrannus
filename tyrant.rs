#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use syntax::ast::{mod, Item, Ident};
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, MacItem, DummyResult};
use syntax::ext::quote::rt::{ToTokens, ExtParseUtils};
use syntax::parse::token;
use syntax::parse::parser::Parser;
use syntax::print::pprust;

use rustc::plugin::Registry;

pub use common::StringMatch;

mod common {
    pub type Match<'a> = (&'a [char], &'a [char]);

    pub struct StringMatch<'a> {
        pub smatch: Option<Match<'a>>,
    }

    impl<'a> Iterator<Match<'a>> for StringMatch<'a> {
        fn next(&mut self) -> Option<Match<'a>> {
            if self.smatch.is_some() {
                let this = self.smatch;
                self.smatch = None;
                this
            } else {
                self.smatch
            }
        }
    }
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


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("parse_string", expand_parse_string);
    reg.register_macro("alt", expand_alt);
}

fn expand_parse_string(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let mut parser = cx.new_parser_from_tts(tts);

    let name = parser.parse_ident();
    let static_ident_str = name.as_str().to_string() + "_str";
    let str_name = Ident::new(token::intern(static_ident_str.as_slice()));

    let x = match parse_string(cx, &mut parser) {
        Some(s) => s,
        None => return DummyResult::any(sp), // TODO: why any?
    };
    let v: Vec<char> = x.as_slice().chars().collect();
    let y = v.as_slice();
    let z = CharVecWrap { vec: v.as_slice() };
    let n = y.len();

    let mut v = vec!();
    v.push( quote_item!(&mut *cx, static $str_name: &'static [char] = $z;).unwrap() );
    v.push( quote_item!(&mut *cx,
        fn $name(inp: &[char]) -> StringMatch {
            if inp.starts_with($str_name) {
                let matched = ($str_name, inp.slice_from($n));
                StringMatch { smatch: Some(matched) }
            } else {
                StringMatch { smatch: None }
            }
        }
    ).unwrap() );


    box MacItems { items: v } as Box<MacResult>
}

struct CharVecWrap<'a> {
    vec: &'a [char]
}

impl<'a> ToTokens for CharVecWrap<'a> {
    fn to_tokens(&self, cx: &ExtCtxt) -> Vec<ast::TokenTree> {
        let mut s = "&".to_string();
        s.push_char('[');
        for (i, c) in self.vec.iter().enumerate() {
            if i != 0 {
                s.push_char(',');
            }
            s.push_char('\'');
            s.push_char(*c);
            s.push_char('\'');
        }
        s.push_char(']');
        cx.parse_tts(s)
    }
}


fn parse_string(cx: &mut ExtCtxt, parser: &mut Parser) -> Option<String> {
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


fn expand_alt(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let mut parser = cx.new_parser_from_tts(tts);

    let one = parser.parse_ident();
    let two = parser.parse_ident();

    let alt_ident_str = "alt_".to_string() + one.as_str() + "_" + two.as_str();
    let alt_ident = Ident::new(token::intern(alt_ident_str.as_slice()));

    MacItem::new(quote_item!(cx, 
        fn $alt_ident(inp: &[char]) -> Chain<StringMatch, StringMatch> {
            $one(inp).chain($two(inp))
        }
    ).unwrap())
}
