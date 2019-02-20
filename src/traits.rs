use std::collections::HashMap;

pub trait Rollable {
    fn roll(&self) -> i32;
    fn plot(&self) -> HashMap<i32, i32>;
}
