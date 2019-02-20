pub mod die;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::die::Die;
    use super::traits::Rollable;
    use std::collections::HashSet;

    #[test]
    fn dice_roll_expected_numbers() {
        let max = 6;
        let expected_outputs = 1..max;
        let die = Die::new(max);

        let actual: HashSet<i32> = (1..100).map(|_| die.roll()).collect();

        for i in expected_outputs {
            assert!(actual.contains(&i))
        }
    }
}
