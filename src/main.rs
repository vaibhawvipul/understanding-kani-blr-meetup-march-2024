fn compute_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
    // handle arithmetic overflow
    // Some((std::num::Wrapping(a) / std::num::Wrapping(b)).0)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_divide() {
        assert_eq!(compute_divide(4, 2).unwrap(), 2);
        assert_eq!(compute_divide(1, 2).unwrap(), 0);
    }

}

#[cfg(kani)]
mod verification {
    #[kani::proof]
    fn test_compute_divide() {
        super::compute_divide(kani::any(), kani::any());
    }
}
