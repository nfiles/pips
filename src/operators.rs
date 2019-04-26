/// An operator that performs some logic on two inputs
pub type BinaryOperator = fn(left: &i32, right: &i32) -> i32;

/// Find the sum of two rolls
pub fn sum(left: &i32, right: &i32) -> i32 {
    (*left) + (*right)
}

/// Find the difference between two rolls
pub fn difference(left: &i32, right: &i32) -> i32 {
    (*left) - (*right)
}

/// Multiple two rolls
pub fn multiply(left: &i32, right: &i32) -> i32 {
    (*left) * (*right)
}

/// Divide two rolls
pub fn divide(left: &i32, right: &i32) -> i32 {
    // TODO: account for divide-by-zero
    (*left) / (*right)
}

/// Take the greater of two rolls
pub fn advantage(left: &i32, right: &i32) -> i32 {
    if *left > *right {
        *left
    } else {
        *right
    }
}

/// Take the lesser of two rolls
pub fn disadvantage(left: &i32, right: &i32) -> i32 {
    if *left < *right {
        *left
    } else {
        *right
    }
}

/// Compare two rolls
///
/// returns:
/// - `1` if left is greater than right
/// - `0` if left is equal to right
/// - `-1` if left is less than right
pub fn compare(left: &i32, right: &i32) -> i32 {
    if *left > *right {
        1
    } else if *left == *right {
        0
    } else {
        -1
    }
}

/// 1 if right is greater than left else 0
pub fn greater_than(left: &i32, right: &i32) -> i32 {
    if *left > *right {
        1
    } else {
        0
    }
}

/// 1 if right is greater than or equal to left else 0
pub fn greater_than_or_equal_to(left: &i32, right: &i32) -> i32 {
    if *left >= *right {
        1
    } else {
        0
    }
}

/// 1 if right is less than left else 0
pub fn less_than(left: &i32, right: &i32) -> i32 {
    if *left > *right {
        1
    } else {
        0
    }
}

/// 1 if right is less than left else 0
pub fn less_than_or_equal_to(left: &i32, right: &i32) -> i32 {
    if *left >= *right {
        1
    } else {
        0
    }
}

/// 1 if right is equal to left else 0
pub fn equal_to(left: &i32, right: &i32) -> i32 {
    if *left > *right {
        1
    } else {
        0
    }
}
