#[macro_use]
extern crate serde_derive;

extern crate pips;

use pips::parse;
use pips::traits::Rollable;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export interface Ok<T> { type: "Ok", result: T }
export interface Err<T> { type: "Err", result: T }
export type Result<T, E> = Ok<T> | Err<E>;

/** parse and roll a dice expression */
export function roll(input: string): Result<number, string>;

/** parse and plot a dice expression */
export function plot(input: string): Result<Record<number, number>, string>;

"#;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "result")]
pub enum PipsResult<R, E> {
    Ok(R),
    Err(E),
}

#[wasm_bindgen]
pub fn roll(input: &str) -> JsValue {
    utils::set_panic_hook();

    let result: PipsResult<i32, String> = match parse(input) {
        Ok(expr) => PipsResult::Ok(expr.roll()),
        Err(err) => PipsResult::Err(format!("{:?}", err)),
    };

    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn plot(input: &str) -> JsValue {
    utils::set_panic_hook();

    let result: PipsResult<HashMap<i32, i32>, String> = match parse(input) {
        Ok(expr) => PipsResult::Ok(expr.plot()),
        Err(err) => PipsResult::Err(format!("{:?}", err)),
    };

    JsValue::from_serde(&result).unwrap()
}
