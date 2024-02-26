mod firstexample;

fn compute_divide(a: i32, b: i32) -> i32 {
    firstexample::compute_divide(a, b)   
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_divide() {
        assert_eq!(compute_divide(4, 2), 2);
        assert_eq!(compute_divide(1, 2), 0);
    }

}

#[cfg(kani)]
mod verification {
    #[kani::proof]
    fn test_compute_divide() {
        super::compute_divide(kani::any(), kani::any());
    }
}
