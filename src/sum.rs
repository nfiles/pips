use std::collections::HashMap;

use crate::traits::Rollable;

pub struct Sum {
    left: Box<dyn Rollable>,
    right: Box<dyn Rollable>,
}

impl Rollable for Sum {
    fn roll(&self) -> i32 {
        self.left.roll() + self.right.roll()
    }

    fn plot(&self) -> HashMap<i32, i32> {
        let left = self.left.plot();
        let right = self.right.plot();

        let mut product: HashMap<i32, i32> = HashMap::new();

        left.iter()
            .flat_map(|(left_value, left_count)| {
                right.iter().map(move |(right_value, right_count)| {
                    (left_value + right_value, left_count * right_count)
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
    fn two_d6_sum_produces_correct_plot() {
        let two_d6 = Sum {
            left: Box::new(Die::new(6)),
            right: Box::new(Die::new(6)),
        };
        let expected: HashMap<i32, i32> = [
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 4),
            (6, 5),
            (7, 6),
            (8, 5),
            (9, 4),
            (10, 3),
            (11, 2),
            (12, 1),
        ]
        .iter()
        .cloned()
        .collect();

        let actual = two_d6.plot();

        assert_eq!(expected, actual);
    }
}
