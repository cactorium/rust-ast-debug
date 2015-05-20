[![Build Status](https://travis-ci.org/cactorium/rust-ast-debug.svg?branch=master)](https://travis-ci.org/cactorium/rust-ast-debug)

# Mistakes were made!
So I kind of impulsively added this to crates.io, without thinking too much about
adding more functionality, so if you've got a project that you want to name
ast-debug and think would be awesome than this, let me know and we'll work out
a way for your project to replace mine.

# Introduction

This compiler plugin takes a function and adds a println! to the top of it
which a string that contains the AST of the function when it's passed through
the Rust parser.

# Example!

```
#![feature(plugin)]
#[plugin] extern crate ast_debug;

ast!(fn foo() {
	println!("Hello world!");
});

```

becomes

```
fn foo() {
	println!(" /* A lot of somewhat pretty printed AST info ends up here! */ ");
	println!("Hello world!");
}
```

# Current Issues
Compiler plugins are currently an unstable feature, so you'll need the nightly
version of Rust to use them. The current code also ends up stripping away a
bit of the function data, namely the ABI, unsafe, and the generics. These will
probably be fixed eventually!
