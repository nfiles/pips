use std::collections::HashMap;

use crate::traits::Rollable;

#[derive(Clone)]
pub struct Constant {
    num: i32,
}

impl Rollable for Constant {
    fn roll(&self) -> i32 {
        self.num
    }

    fn plot(&self) -> HashMap<i32, i32> {
        [(self.num, 1)].iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn constant_produces_expected_number() {
        let num = 10;
        let times = 100;
        let num_roll = Constant { num };

        let rolls: HashSet<i32> = (0..times).map(|_| num_roll.roll()).collect();

        assert_eq!(1, rolls.len());
        assert!(rolls.contains(&num));
    }

    #[test]
    fn constant_produces_correct_plot() {
        let four = Constant { num: 4 };
        let expected: HashMap<i32, i32> = [(4, 1)].iter().cloned().collect();

        let actual = four.plot();

        assert_eq!(expected, actual);
    }
}
