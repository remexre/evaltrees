/// A `linefeed` Terminal based on xterm.js.
#[wasm_bindgen]
pub struct XTermJS {
    clear: Box<Fn()>,
    size: Box<Fn() -> (u32, u32)>,
}
