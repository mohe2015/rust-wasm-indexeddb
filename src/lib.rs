use wasm_bindgen::{JsCast, prelude::*};
use web_sys::IdbOpenDbRequest;
use web_sys::window;
use web_sys::console;
use web_sys::IdbRequest;
use web_sys::Event;
use web_sys::IdbDatabase;
use std::ops::Deref;

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&format!("Hello, {}!", name).into());

    let window = window().unwrap();
    let idb = window.indexed_db().unwrap().unwrap();

    let open_request = idb.open_with_u32("test", 2).unwrap();

    let a = Closure::wrap(Box::new(|event: Event| {
        console::log_1( &event);
        //let result = idbRequest.result().unwrap();
        let target = event.target().unwrap();
        let new_open_request: IdbOpenDbRequest = target.dyn_into::<IdbOpenDbRequest>().unwrap();

        let db = new_open_request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
        console::log_1( &db);
        
    }) as Box<dyn FnMut(Event)>);

    let b = Closure::wrap(Box::new(|event: Event| {
        console::log_1( &event);
        //let result = idbRequest.result().unwrap();
        let target = event.target().unwrap();
        let new_open_request: IdbOpenDbRequest = target.dyn_into::<IdbOpenDbRequest>().unwrap();

        let db = new_open_request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
        console::log_1( &db);

        let object_store = db.create_object_store("test").unwrap();
    }) as Box<dyn FnMut(Event)>);

    open_request.set_onupgradeneeded(Some(b.as_ref().unchecked_ref()));

    open_request.set_onsuccess(Some(a.as_ref().unchecked_ref()));

    a.forget();
    b.forget();
}
