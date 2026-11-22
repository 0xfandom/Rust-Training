//! Overflow-safe modular exponentiation by repeated squaring.
//!
//! Computes `base^exponent mod modulus` in O(log exponent) time and O(1)
//! additional space. Multiplication uses `u128` intermediates so the full
//! `u64` input range is supported.

/// Returns `base^exponent mod modulus`, or `None` if the modulus is zero.
///
/// An exponent of zero yields `1 % modulus`, including when the base is zero.
pub fn mod_pow(base: u64, mut exponent: u64, modulus: u64) -> Option<u64> {
    if modulus == 0 {
        return None;
    }

    let modulus = u128::from(modulus);
    let mut factor = u128::from(base) % modulus;
    let mut result = 1 % modulus;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result * factor % modulus;
        }
        exponent >>= 1;
        if exponent > 0 {
            factor = factor * factor % modulus;
        }
    }

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::mod_pow;

    #[test]
    fn rejects_zero_modulus() {
        assert_eq!(mod_pow(2, 10, 0), None);
        assert_eq!(mod_pow(0, 0, 0), None);
    }

    #[test]
    fn handles_zero_exponent_and_modulus_one() {
        assert_eq!(mod_pow(7, 0, 13), Some(1));
        assert_eq!(mod_pow(0, 0, 13), Some(1));
        assert_eq!(mod_pow(7, 0, 1), Some(0));
        assert_eq!(mod_pow(7, 99, 1), Some(0));
    }

    #[test]
    fn handles_zero_base_and_known_results() {
        assert_eq!(mod_pow(0, 9, 17), Some(0));
        assert_eq!(mod_pow(2, 10, 1000), Some(24));
        assert_eq!(mod_pow(3, 5, 13), Some(9));
        assert_eq!(mod_pow(10, 1, 7), Some(3));
    }

    #[test]
    fn handles_large_operands_without_overflow() {
        assert_eq!(mod_pow(u64::MAX, 2, u64::MAX), Some(0));
        assert_eq!(mod_pow(u64::MAX - 1, 2, u64::MAX), Some(1));
        assert_eq!(mod_pow(u64::MAX, u64::MAX, 2), Some(1));
    }

    #[test]
    fn matches_repeated_multiplication_on_small_inputs() {
        for modulus in 1_u64..20 {
            for base in 0_u64..20 {
                for exponent in 0_u64..16 {
                    let mut expected = 1_u128 % u128::from(modulus);
                    for _ in 0..exponent {
                        expected = expected * u128::from(base) % u128::from(modulus);
                    }
                    assert_eq!(
                        mod_pow(base, exponent, modulus),
                        Some(expected as u64),
                        "base={base}, exponent={exponent}, modulus={modulus}"
                    );
                }
            }
        }
    }
}
