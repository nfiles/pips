use std::collections::HashMap;

use crate::traits::Rollable;

pub struct Disadvantage {
    left: Box<dyn Rollable>,
    right: Box<dyn Rollable>,
}

impl Rollable for Disadvantage {
    fn roll(&self) -> i32 {
        self.left.roll() * self.right.roll()
    }

    fn plot(&self) -> HashMap<i32, i32> {
        let left = self.left.plot();
        let right = self.right.plot();

        let mut product: HashMap<i32, i32> = HashMap::new();

        left.iter()
            .flat_map(|(left_value, left_count)| {
                right.iter().map(move |(right_value, right_count)| {
                    let value = if left_value < right_value {
                        left_value
                    } else {
                        right_value
                    };
                    (*value, left_count * right_count)
                })
            })
            .for_each(|(value, count)| {
                *product.entry(value).or_insert(0) += count;
            });

        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::die::Die;

    #[test]
    fn disadvantage_produces_correct_plot() {
        let two_d4 = Disadvantage {
            left: Box::new(Die::new(4)),
            right: Box::new(Die::new(4)),
        };
        // 1 1 -> 1
        // 1 2 -> 1
        // 1 3 -> 1
        // 1 4 -> 1
        // 2 1 -> 1
        // 2 2 -> 2
        // 2 3 -> 2
        // 2 4 -> 2
        // 3 1 -> 1
        // 3 2 -> 2
        // 3 3 -> 3
        // 3 4 -> 3
        // 4 1 -> 1
        // 4 2 -> 2
        // 4 3 -> 3
        // 4 4 -> 4
        let expected: HashMap<i32, i32> =
            [(1, 7), (2, 5), (3, 3), (4, 1)].iter().cloned().collect();

        let actual = two_d4.plot();

        assert_eq!(expected, actual);
    }
}
