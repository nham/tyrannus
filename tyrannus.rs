#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr, DummyResult};
use syntax::parse::{mod, token};
use rustc::plugin::Registry;
use common::StringMatch;

mod common {
    type Match<'a> = (&'a [char], &'a [char]);

    pub struct StringMatch<'a> {
        smatch: Option<Match<'a>>,
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

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("parse_string", expand_parse_string);
}


fn expand_parse_string(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let x = match parse_string(cx, tts) {
        Some(s) => s,
        None => return DummyResult::any(sp), // TODO: why any?
    };
    let n: uint = x + 5;
    MacExpr::new(quote_expr!(cx, $n))
}


fn parse(cx: &mut ExtCtxt, tts: &[ast::TokenTree]) -> Option<uint> {
    use syntax::print::pprust;

    let mut parser = cx.new_parser_from_tts(tts);
    let arg = parser.parse_expr();

    // libregex uses cx.expander().fold_expr(parser.parse_expr());

    match arg.node {
        ast::ExprLit(spanned) => {
            match spanned.node {
                ast::LitStr(ref s, _) => {
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
