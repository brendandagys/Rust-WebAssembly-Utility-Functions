use wasm_bindgen::prelude::*;

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
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hi there, {}!", name));
}

// =========================================
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn add(a: u32, b: u32) -> u32 {
//     a + b
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

// =========================================

// // use std::future;

// use async_std::future::Future;
// // use async_std::task;
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

// #[wasm_bindgen]
// pub struct MapAsync();

// impl MapAsync {
//     pub async fn map_async<T, U>(list: Vec<T>, f: &dyn Fn(&T, usize, &[T]) -> U) -> Vec<U>
//     where
//         U: Future<Output = U>,
//         // F: Fn(&T, usize, &[T]) -> U,
//     {
//         let mut result = Vec::with_capacity(list.len());

//         for (i, item) in list.iter().enumerate() {
//             result.push(f(item, i, &list));
//         }

//         futures::future::join_all(result).await
//     }
// }

// // #[wasm_bindgen]
// pub async fn map_async<T, U>(list: Vec<T>, f: &dyn Fn(&T, usize, &[T]) -> U) -> Vec<U>
// where
//     U: Future<Output = U>,
//     // F: Fn(&T, usize, &[T]) -> U,
// {
//     let mut result = Vec::with_capacity(list.len());

//     for (i, item) in list.iter().enumerate() {
//         result.push(f(item, i, &list));
//     }

//     futures::future::join_all(result).await
// }

// // where
// //     T: std::marker::Send + std::marker::Sync,
// //     U: Future<Output = U>,
// //     U: std::marker::Send + std::marker::Sync,
// //     F: Fn(&T, usize, Vec<T>) -> U + std::marker::Send + std::marker::Sync,
// // {
// //     let mut tasks: Vec<task::JoinHandle<U>> = Vec::with_capacity(list.len());

// //     for (i, item) in list.iter().enumerate() {
// //         tasks.push(task::spawn(async move { f(item, i, list).await }))
// //     }

// //     task::block_on(async {
// //         for t in tasks {
// //             t.await;
// //         }
// //     });

// //     tasks
