//! Greatest common divisor using the iterative Euclidean algorithm.

/// Returns the greatest common divisor of two unsigned integers.
///
/// By convention, `gcd(0, 0)` returns `0`. For nonzero inputs the algorithm
/// takes O(log(min(a, b))) time and O(1) auxiliary space.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn zero_inputs() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 42), 42);
        assert_eq!(gcd(42, 0), 42);
    }

    #[test]
    fn coprime_and_common_factors() {
        assert_eq!(gcd(35, 64), 1);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(24, 54), 6);
    }

    #[test]
    fn equal_inputs() {
        assert_eq!(gcd(97, 97), 97);
    }

    #[test]
    fn maximum_unsigned_values() {
        assert_eq!(gcd(u64::MAX, 0), u64::MAX);
        assert_eq!(gcd(u64::MAX, u64::MAX), u64::MAX);
        assert_eq!(gcd(u64::MAX, 2), 1);
    }
}
