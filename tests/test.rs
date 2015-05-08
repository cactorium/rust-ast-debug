#![feature(plugin, collections)]

#![plugin(ast_debug)]

ast!(fn foo() -> Box<Option<u32>> {
    let mut f = 4;
    let g: Option<String> = Some(String::from_str("fooo"));
    f = f + 2u32;
    if let Some(_) = g {
        Box::new(Some(f))
    } else {
        Box::new(None)
    }
});

// Running cargo test -- --nocapture lets you see the output
#[test]
fn test_foo() {
    foo();
}
