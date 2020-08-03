extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {


    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let val = document.create_element("Button")?;
    val.set_attribute("onclick", "alert('dd')");
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    Ok(())
}
