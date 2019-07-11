extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, js_name = setAttribute)]
    fn set_attribute(this: &Element, key: &str, value: &str);

    // #[wasm_bindgen(method, setter = innerHTML)]
    // fn set_inner(this: &Element, html: &str);

    // #[wasm_bindgen(method, getter)]
    // fn body(this: &HTMLDocument) -> Element;

    // #[wasm_bindgen(method, js_name = appendChild)]
    // fn append(this: &Element, item: Element);
}

#[wasm_bindgen]
pub fn create_stuff() {
    let script = document.createElement("script");
    script.set_attribute("src", "https://apis.google.com/js/platform.js");
    script.set_attribute("id", "youtube-subscribe");

    document.body().append(script);

    // let div = document.createElement("div");
    // let p = document.createElement("p");
    // let _span = document.createElement("span");

    // p.set_inner("Hello from the Rust to JavaScript");
    // div.append(p);
    // document.body().append(div);
}

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, from rust {}!", name));
// }

// fn main() {
//     println!("Would it work?");
// }
