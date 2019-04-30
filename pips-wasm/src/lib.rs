#[macro_use]
extern crate serde_derive;

extern crate pips;

use pips::parse;
use pips::traits::Rollable;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;

mod utils;

#[derive(Serialize)]
pub struct RollResult<T> {
    pub success: bool,
    pub result: T,
    pub err: String,
}

#[derive(Serialize)]
pub enum PipsResult<R, E> {
    Ok(R),
    Err(E),
}

// #[derive(Serialize)]
// pub struct PlotResult {
//     pub success: bool,
//     pub result: HashMap<i32, i32>,
//     pub err: String,
// }

#[wasm_bindgen]
pub fn roll(input: &str) -> JsValue {
    // let result: RollResult<i32> = match parse(input) {
    //     Ok(expr) => RollResult {
    //         success: true,
    //         result: expr.roll(),
    //         err: "".to_string(),
    //     },
    //     Err(err) => RollResult {
    //         success: false,
    //         result: 0,
    //         err: format!("{:?}", err),
    //     },
    // };
    let result: PipsResult<i32, String> = match parse(input) {
        Ok(expr) => PipsResult::Ok(expr.roll()),
        Err(err) => PipsResult::Err(format!("{:?}", err)),
    };

    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn plot(input: &str) -> JsValue {
    let result: RollResult<HashMap<i32, i32>> = match parse(input) {
        Ok(expr) => RollResult {
            success: true,
            result: expr.plot(),
            err: "".to_string(),
        },
        Err(err) => RollResult {
            success: false,
            result: HashMap::new(),
            err: format!("{:?}", err),
        },
    };

    JsValue::from_serde(&result).unwrap()
}
