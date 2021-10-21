use wasm_bindgen::prelude::*;
use web_sys::IdbFactory;
use web_sys::window;

#[wasm_bindgen]
pub fn greet(name: &str) {
    //console::log_1(&format!("Hello, {}!", name).into());

    let window = window().unwrap();
    let idb = window.indexed_db().unwrap().unwrap();

    idb.delete_database("test").unwrap();

    idb.delete_database("test").unwrap();
}
