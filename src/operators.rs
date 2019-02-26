pub type BinaryOperator = fn(left: &i32, right: &i32) -> i32;

pub fn sum(left: &i32, right: &i32) -> i32 {
    (*left) + (*right)
}

pub fn multiply(left: &i32, right: &i32) -> i32 {
    (*left) * (*right)
}

pub fn advantage(left: &i32, right: &i32) -> i32 {
    if *left > *right {
        *left
    } else {
        *right
    }
}

pub fn disadvantage(left: &i32, right: &i32) -> i32 {
    if *left < *right {
        *left
    } else {
        *right
    }
}

pub fn compare(left: &i32, right: &i32) -> i32 {
    if *left > *right {
        1
    } else if *left == *right {
        0
    } else {
        -1
    }
}
