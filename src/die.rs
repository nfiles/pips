use rand::Rng;
use std::collections::HashMap;

use super::traits::Rollable;

pub struct Die {
    max: i32,
}

impl Die {
    pub fn new(max: i32) -> Die {
        if max <= 0 {
            panic!("max is invalid")
        }

        Die { max }
    }
}

impl Rollable for Die {
    fn roll(&self) -> i32 {
        rand::thread_rng().gen_range(1, self.max + 1)
    }

    fn plot(&self) -> HashMap<i32, i32> {
        // a single die has an equal chance to roll any of its sides
        (1..self.max).map(|i| (i, 1)).collect()
    }
}
