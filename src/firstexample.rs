use std::num::Wrapping;

pub fn compute_divide(a: i32, b: i32) -> i32 {
    a / b
}

pub fn compute_divide_div_zero(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

pub fn compute_divide_wrapped(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some((std::num::Wrapping(a) / std::num::Wrapping(b)).0)
}