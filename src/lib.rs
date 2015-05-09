#![feature(quote, plugin_registrar)]
#![crate_type = "dylib"]
#![feature(rustc_private, collections, quote)]

#![feature(core)]
extern crate core;
extern crate syntax;
extern crate rustc;

use core::default::Default;

use syntax::codemap::Span;
// use syntax::parse::token;
use syntax::ast;
use syntax::ast::{TokenTree, Item};
use syntax::ext::base::{ExtCtxt, MacResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn registrar(reg: &mut rustc::plugin::Registry) {
  reg.register_macro("ast", expand_ast);
}

fn format_braces(unformatted_str: String) -> String {
    let mut tab_level = 0;
    let rbrace = unformatted_str.replace("{", "{\n");
    let lbrace = rbrace.replace("}", "\n}");
    let comma = lbrace.replace(",",  ",\n");
    let lbrac = comma.replace("[", "[\n");
    let lines_str = lbrac.lines();
    let line_map = lines_str.map(|ln: &str| -> (String, usize) {
            for ch in ln.chars() {
                if ch == '}' || ch == ']' {
                    tab_level = tab_level - 1;
                }
            }
            let ret = (String::from_str(ln.trim()), tab_level);
            for ch in ln.chars() {
                if ch == '{' || ch == '(' || ch == '[' {
                    tab_level = tab_level + 1;
                }
                if ch == ')' {
                    tab_level = tab_level - 1;
                }
            }
            ret
        }).map(|tup: (String, usize)| -> String {
            let (ln, lvl) = tup;
            let mut strng = String::from_str("\n");
            for _ in 0..lvl {
                strng.push_str("  ");
            }
            strng.push_str(&*ln);
            strng
        });
    let mut result = String::new();
    for ln in line_map {
        result.push_str(&*ln);
    }
    result
}

fn expand_ast(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult+'static> {
    let mut parser = cx.new_parser_from_tts(args);
    let old_fn = parser.parse_item();

    let mut new_items = SmallVector::<P<Item>>::zero();

    if let Some(really_old_fn) = old_fn {
        match really_old_fn.node {
            ast::ItemFn(ref decl, _, _, _, ref block) => {

                let mut new_contents = (**block).clone();
                let block_debug = format!("{:?}", block);
                let formatted_block_debug = format_braces(block_debug);
                let debug_stmt = quote_stmt!(&mut *cx,
                    println!("{}", $formatted_block_debug);
                );
                if let Some(rly_debug_stmt) = debug_stmt {
                    new_contents.stmts.insert(0, rly_debug_stmt.clone());
                }
                let out_ty = match decl.output {
                    ast::Return(ref pty) => (*pty).clone(),
                    ast::NoReturn(ref span) | 
                        ast::DefaultReturn(ref span) => cx.ty(*span, ast::TyTup(Vec::new()))
                };
                let new_fn = cx.item_fn(sp, really_old_fn.ident, decl.inputs.clone(),
                                        out_ty, P(new_contents));
                new_items.push(new_fn);
                MacEager::items(new_items)
            }
            _ => {
                cx.span_err(sp, "ast is only permissible on functions");
                Box::new(MacEager::default())
            }
        }
    } else {
        cx.span_err(sp, "Unable to parse contents into an item");
        Box::new(MacEager::default())
    }
}

#[test]
fn it_works() {
}
