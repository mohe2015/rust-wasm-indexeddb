use js_sys::Function;
use js_sys::JSON;
use js_sys::Promise;
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::IdbObjectStore;
use web_sys::IdbOpenDbRequest;
use web_sys::window;
use web_sys::console;
use web_sys::Event;
use web_sys::IdbDatabase;
use web_sys::IdbTransactionMode;
use js_sys::Array;

pub async fn put_with_key(object_store: &IdbObjectStore, js_value: &JsValue, key: &JsValue) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let promise = Promise::new(&mut |resolve, reject| {
        let request = object_store.put_with_key(js_value, key).unwrap();

        request.set_onsuccess(Some(&resolve));
        request.set_onerror(Some(&reject));
    });

    let result = wasm_bindgen_futures::JsFuture::from(promise);
    result.await
}

pub async fn open(name: &str, version: u32) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let promise = Promise::new(&mut |resolve: Function, reject: Function| {

        let window = window().unwrap();
        let idb = window.indexed_db().unwrap().unwrap();

        let open_request = idb.open_with_u32(name, version).unwrap();

        let onsuccess = Closure::wrap(Box::new(|event: Event| {
           
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

        open_request.set_onsuccess(Some(&resolve)); // onsuccess.as_ref().unchecked_ref()

        onsuccess.forget();
        onupgradeneeded.forget();
    });

    let result = wasm_bindgen_futures::JsFuture::from(promise);
    result.await
}

#[wasm_bindgen]
pub async fn greet() {
    console_error_panic_hook::set_once();

    console::log_1(&format!("Hello!").into());

    let event = open("test", 2).await.unwrap().dyn_into::<Event>().unwrap();

    console::log_1( &event);
    let target = event.target().unwrap();
    let new_open_request: IdbOpenDbRequest = target.dyn_into::<IdbOpenDbRequest>().unwrap();

    let db = new_open_request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
    console::log_1( &db);

    let store_names = JsValue::from(Array::from_iter([JsValue::from("test")].iter()));

    let transaction = db.transaction_with_str_sequence_and_mode(&store_names, IdbTransactionMode::Readwrite).unwrap();

    let object_store = transaction.object_store("test").unwrap();

    put_with_key(&object_store, &JSON::parse(r#"{"id":1}"#).unwrap(), &JsValue::from_f64(1.0)).await.unwrap();

    put_with_key(&object_store, &JSON::parse(r#"{"id":2}"#).unwrap(), &JsValue::from_f64(2.0)).await.unwrap();

    console::log_1(&format!("Done!").into());
}
