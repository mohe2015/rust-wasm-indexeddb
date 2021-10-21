use js_sys::JSON;
use js_sys::Promise;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::IdbOpenDbRequest;
use web_sys::window;
use web_sys::console;
use web_sys::Event;
use web_sys::IdbDatabase;
use web_sys::IdbTransactionMode;
use js_sys::Array;

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&format!("Hello, {}!", name).into());

    let window = window().unwrap();
    let idb = window.indexed_db().unwrap().unwrap();

    let open_request = idb.open_with_u32("test", 2).unwrap();

    let onsuccess = Closure::wrap(Box::new(|event: Event| {
        console::log_1( &event);
        //let result = idbRequest.result().unwrap();
        let target = event.target().unwrap();
        let new_open_request: IdbOpenDbRequest = target.dyn_into::<IdbOpenDbRequest>().unwrap();

        let db = new_open_request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
        console::log_1( &db);

        let store_names = JsValue::from(Array::from_iter([JsValue::from("test")].iter()));

        let transaction = db.transaction_with_str_sequence_and_mode(&store_names, IdbTransactionMode::Readwrite).unwrap();

        let object_store = transaction.object_store("test").unwrap();

        let promise = Promise::new(&mut |resolve, reject| {
            let request = object_store.put(&JSON::parse(r#"dfs"#).unwrap()).unwrap();

            request.set_onsuccess(Some(&resolve));
            request.set_onerror(Some(&reject));
        });

        let result = wasm_bindgen_futures::JsFuture::from(promise);
    }) as Box<dyn FnMut(Event)>);

    let onupgradeneeded = Closure::wrap(Box::new(|event: Event| {
        console::log_1( &event);
        //let result = idbRequest.result().unwrap();
        let target = event.target().unwrap();
        let new_open_request: IdbOpenDbRequest = target.dyn_into::<IdbOpenDbRequest>().unwrap();

        let db = new_open_request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
        console::log_1( &db);

        let _object_store = db.create_object_store("test").unwrap();
    }) as Box<dyn FnMut(Event)>);

    open_request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));

    open_request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));

    onsuccess.forget();
    onupgradeneeded.forget();
}
