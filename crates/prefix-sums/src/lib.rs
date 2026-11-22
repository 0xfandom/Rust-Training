//! Immutable prefix sums for repeated range-sum queries.
//!
//! Building the table takes O(n) time and O(n) space. Each query is O(1).
//! Accumulating `i64` inputs into `i128` avoids overflow for slices
//! addressable by Rust's `usize` on supported 32- and 64-bit targets.

/// Prefix sums with a leading zero, so the sum of `[start, end)`
/// is `prefix[end] - prefix[start]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixSums {
    prefix: Vec<i128>,
}

impl PrefixSums {
    /// Builds prefix sums from signed 64-bit integers.
    pub fn new(values: &[i64]) -> Self {
        let mut prefix = Vec::with_capacity(values.len() + 1);
        prefix.push(0);
        for &value in values {
            let next = prefix[prefix.len() - 1] + i128::from(value);
            prefix.push(next);
        }
        Self { prefix }
    }

    /// Returns the sum of the half-open range `[start, end)`.
    ///
    /// Empty ranges are valid. Reversed or out-of-bounds ranges return `None`.
    pub fn sum(&self, start: usize, end: usize) -> Option<i128> {
        if start > end || end >= self.prefix.len() {
            return None;
        }
        Some(self.prefix[end] - self.prefix[start])
    }

    /// Returns the number of input values.
    pub fn len(&self) -> usize {
        self.prefix.len() - 1
    }

    /// Returns whether the original slice was empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixSums;

    #[test]
    fn empty_input_has_one_valid_range() {
        let sums = PrefixSums::new(&[]);
        assert!(sums.is_empty());
        assert_eq!(sums.len(), 0);
        assert_eq!(sums.sum(0, 0), Some(0));
        assert_eq!(sums.sum(0, 1), None);
    }

    #[test]
    fn full_partial_and_empty_ranges() {
        let sums = PrefixSums::new(&[3, 5, 7, 11]);
        assert_eq!(sums.len(), 4);
        assert!(!sums.is_empty());
        assert_eq!(sums.sum(0, 4), Some(26));
        assert_eq!(sums.sum(1, 3), Some(12));
        assert_eq!(sums.sum(2, 2), Some(0));
        assert_eq!(sums.sum(4, 4), Some(0));
    }

    #[test]
    fn signed_values_and_repeated_queries() {
        let sums = PrefixSums::new(&[-8, 3, -2, 10]);
        assert_eq!(sums.sum(0, 3), Some(-7));
        assert_eq!(sums.sum(1, 4), Some(11));
        assert_eq!(sums.sum(0, 3), Some(-7));
    }

    #[test]
    fn invalid_ranges_do_not_panic() {
        let sums = PrefixSums::new(&[1, 2]);
        assert_eq!(sums.sum(2, 1), None);
        assert_eq!(sums.sum(0, 3), None);
        assert_eq!(sums.sum(usize::MAX, usize::MAX), None);
    }

    #[test]
    fn wide_accumulator_handles_integer_limits() {
        let sums = PrefixSums::new(&[i64::MAX, i64::MAX, i64::MIN]);
        assert_eq!(sums.sum(0, 2), Some(i128::from(i64::MAX) * 2));
        assert_eq!(sums.sum(0, 3), Some(i128::from(i64::MAX) - 1));
        assert_eq!(sums.sum(2, 3), Some(i128::from(i64::MIN)));
    }
}
