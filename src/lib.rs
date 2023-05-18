use serde_json::Value;
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::HashMap;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");

    // // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_inner_html("Hello from Rust!");

    // body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn keys(obj: JsValue) -> JsValue {
    let serde_value = from_value(obj).unwrap();
    let map: HashMap<String, Value> = serde_json::from_value(serde_value).unwrap();
    let result: Vec<&String> = map.keys().collect();
    return to_value(&result).unwrap();
}

#[wasm_bindgen]
pub fn compact_array(arr: Box<[JsValue]>) -> Box<[JsValue]> {
    (*arr)
        .iter()
        .filter(|&val| !(*val == JsValue::NULL || *val == JsValue::UNDEFINED))
        .cloned()
        .collect::<Box<[JsValue]>>()
}

#[wasm_bindgen]
pub fn distinct(arr: Box<[JsValue]>) -> Box<[JsValue]> {
    log("Hello, `distinct` is running...");
    let mut result = Vec::new();

    for val in arr.iter() {
        log(&format!("{:?}", val));
        if !result.contains(val) {
            result.push(val.clone())
        }
    }

    result.into_boxed_slice()
}

#[wasm_bindgen]
pub fn int_val(val: JsValue) -> JsValue {
    if val.is_string() {
        log(&val.as_string().unwrap());
        match val.as_string().unwrap().parse::<i32>() {
            Ok(res) => return JsValue::from(res),
            Err(_) => return JsValue::NULL,
        }
    }

    if let Some(parsed) = val.as_f64() {
        JsValue::from(parsed as i32)
    } else {
        JsValue::NULL
    }
}

#[wasm_bindgen]
pub fn float_val(val: JsValue) -> JsValue {
    if val.is_string() {
        match val.as_string().unwrap().parse::<f64>() {
            Ok(res) => return res.into(),
            Err(_) => return JsValue::NULL,
        }
    }

    if let Some(parsed) = val.as_f64() {
        JsValue::from(parsed)
    } else {
        JsValue::NULL
    }
}
