#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate evaltrees;
extern crate futures;
extern crate wasm_bindgen;

pub mod io;

use wasm_bindgen::prelude::*;

use io::IoFacade;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(f: &Closure<FnMut()>, time: u32);
}

#[wasm_bindgen(module = "./../ui.js")]
extern "C" {
    fn write(s: &str);
}

#[wasm_bindgen]
pub struct Repl {
    io: IoFacade,
}

#[wasm_bindgen]
impl Repl {
    /// Creates a new REPL instance.
    pub fn new() -> Repl {
        Repl {
            io: IoFacade::new(),
        }
    }

    /// Writes a string into the REPL.
    pub fn write(&mut self, s: &str) {
        self.io.put(s);
    }
}
