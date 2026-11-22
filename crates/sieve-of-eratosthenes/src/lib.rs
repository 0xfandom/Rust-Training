//! Prime generation with the Sieve of Eratosthenes.
//!
//! Runs in O(n log log n) time and O(n) auxiliary space for a limit of n.
//! The composite table allocates one entry per integer through the limit.

/// Returns all prime numbers less than or equal to `limit`, in ascending order.
///
/// Limits below 2 produce an empty vector. Memory use grows linearly with the
/// limit, so callers should choose a bound that fits available memory.
pub fn primes_up_to(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }

    let mut composite = vec![false; limit + 1];
    let mut prime = 2;

    // Division avoids overflowing when checking whether prime^2 exceeds limit.
    while prime <= limit / prime {
        if !composite[prime] {
            let mut multiple = prime * prime;
            while multiple <= limit {
                composite[multiple] = true;
                // Do not compute a multiple beyond the inclusive bound.
                if prime > limit - multiple {
                    break;
                }
                multiple += prime;
            }
        }
        prime += 1;
    }

    (2..=limit).filter(|&n| !composite[n]).collect()
}

#[cfg(test)]
mod tests {
    use super::primes_up_to;

    #[test]
    fn empty_for_limits_below_two() {
        assert!(primes_up_to(0).is_empty());
        assert!(primes_up_to(1).is_empty());
    }

    #[test]
    fn includes_prime_limit() {
        assert_eq!(primes_up_to(2), vec![2]);
        assert_eq!(primes_up_to(3), vec![2, 3]);
    }

    #[test]
    fn excludes_composites_and_preserves_order() {
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(
            primes_up_to(30),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn counts_primes_through_one_thousand() {
        let primes = primes_up_to(1_000);
        assert_eq!(primes.len(), 168);
        assert_eq!(primes.first(), Some(&2));
        assert_eq!(primes.last(), Some(&997));
        assert!(primes.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn matches_trial_division_for_small_limits() {
        let expected: Vec<_> = (2..=200)
            .filter(|&n| (2..n).all(|divisor| n % divisor != 0))
            .collect();
        assert_eq!(primes_up_to(200), expected);
    }
}
