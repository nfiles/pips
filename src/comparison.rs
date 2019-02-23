use std::collections::HashMap;

use crate::traits::Rollable;

type Comparator<T> = fn(check: &T, roll: &T) -> bool;

pub struct Check<'a> {
    expression: &'a Rollable,
    predicate: Comparator<i32>,
    value: i32,
}

impl<'a> Rollable for Check<'a> {
    fn roll(&self) -> i32 {
        let roll = self.expression.roll();
        if (self.predicate)(&self.value, &roll) {
            1
        } else {
            0
        }
    }

    fn plot(&self) -> HashMap<i32, i32> {
        let mut result: HashMap<i32, i32> = HashMap::new();

        self.expression.plot().iter().for_each(|(value, count)| {
            let success = if (self.predicate)(&self.value, value) {
                1
            } else {
                0
            };
            *result.entry(success).or_insert(0) += count;
        });

        result
    }
}
