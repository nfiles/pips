use rand::Rng;
use std::collections::HashMap;

use crate::traits::Rollable;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn dice_roll_expected_numbers() {
        let max = 6;
        let num_rolls = 100;
        let die = Die::new(max);

        let actual: HashSet<i32> = (1..num_rolls).map(|_| die.roll()).collect();

        for i in 1..max {
            assert!(actual.contains(&i))
        }
    }
}
