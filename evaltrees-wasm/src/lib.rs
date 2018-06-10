#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate evaltrees;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

/// The main object through which interaction occurs.
#[wasm_bindgen]
pub struct Repl {
    foo: u32,
}

#[wasm_bindgen]
impl Repl {
    /// Creates a new Repl instance.
    pub fn new() -> Repl {
        Repl { foo: 4 }
    }

    /// Prints "Hello, world!".
    pub fn hullo(&mut self) -> String {
        self.foo += 1;
        format!("Hello #{}", self.foo)
    }
}
