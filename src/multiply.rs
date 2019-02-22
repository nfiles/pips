use std::collections::HashMap;

use crate::traits::Rollable;

pub struct Multiply {
    left: Box<dyn Rollable>,
    right: Box<dyn Rollable>,
}

impl Rollable for Multiply {
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
                    (left_value * right_value, left_count * right_count)
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
    fn multiply_produces_correct_plot() {
        let two_d4 = Multiply {
            left: Box::new(Die::new(4)),
            right: Box::new(Die::new(4)),
        };
        let expected: HashMap<i32, i32> = [
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (6, 2),
            (8, 2),
            (9, 1),
            (12, 2),
            (16, 1),
        ]
        .iter()
        .cloned()
        .collect();

        let actual = two_d4.plot();

        assert_eq!(expected, actual);
    }
}
