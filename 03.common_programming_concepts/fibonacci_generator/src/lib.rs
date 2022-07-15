pub fn fibonacci_generator(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    return fibonacci_generator(n - 1) + fibonacci_generator(n - 2);
}

#[cfg(test)]
mod fibonacci_generator_tests {
    use super::*;

    #[test]
    fn n_zero_returns_zero() {
        assert_eq!(0, fibonacci_generator(0));
    }

    #[test]
    fn n_one_returns_one() {
        assert_eq!(1, fibonacci_generator(1));
    }

    #[test]
    fn n_two_returns_one() {
        assert_eq!(1, fibonacci_generator(2));
    }

    #[test]
    fn n_nine_returns_thirty_four() {
        assert_eq!(34, fibonacci_generator(9));
    }
}
