extern crate chrono;
extern crate mime;
extern crate failure;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate toml;
extern crate config;
extern crate env_logger;
extern crate itertools;

#[macro_use] extern crate is_match;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
