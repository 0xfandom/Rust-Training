//! Length of a strictly increasing subsequence.

/// Returns the maximum length of a strictly increasing subsequence.
///
/// `tails[i]` is the smallest possible final value of an increasing
/// subsequence of length `i + 1` seen so far. Replacing a tail cannot
/// reduce the length of a future extension.
///
/// Time: O(n log n). Auxiliary space: O(n).
pub fn lis_length(values: &[i64]) -> usize {
    let mut tails = Vec::with_capacity(values.len());

    for &value in values {
        let position = tails.partition_point(|&tail| tail < value);
        if position == tails.len() {
            tails.push(value);
        } else {
            tails[position] = value;
        }
    }

    tails.len()
}

#[cfg(test)]
mod tests {
    use super::lis_length;

    fn reference(values: &[i64]) -> usize {
        let mut lengths = vec![1; values.len()];
        for i in 0..values.len() {
            for j in 0..i {
                if values[j] < values[i] {
                    lengths[i] = lengths[i].max(lengths[j] + 1);
                }
            }
        }
        lengths.into_iter().max().unwrap_or(0)
    }

    #[test]
    fn handles_empty_and_singleton_inputs() {
        assert_eq!(lis_length(&[]), 0);
        assert_eq!(lis_length(&[7]), 1);
    }

    #[test]
    fn handles_monotone_inputs() {
        assert_eq!(lis_length(&[1, 2, 3, 4]), 4);
        assert_eq!(lis_length(&[4, 3, 2, 1]), 1);
    }

    #[test]
    fn does_not_count_equal_values_twice() {
        assert_eq!(lis_length(&[2, 2, 2, 2]), 1);
        assert_eq!(lis_length(&[1, 2, 2, 3]), 3);
    }

    #[test]
    fn handles_negative_values_and_integer_limits() {
        assert_eq!(lis_length(&[i64::MIN, -1, 0, i64::MAX]), 4);
        assert_eq!(lis_length(&[i64::MAX, i64::MIN, 0]), 2);
    }

    #[test]
    fn finds_noncontiguous_subsequences() {
        assert_eq!(lis_length(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn matches_quadratic_reference_on_small_inputs() {
        for len in 0..=6 {
            let cases = 3_usize.pow(len);
            for mut encoded in 0..cases {
                let mut values = Vec::with_capacity(len);
                for _ in 0..len {
                    values.push((encoded % 3) as i64 - 1);
                    encoded /= 3;
                }
                assert_eq!(lis_length(&values), reference(&values), "{values:?}");
            }
        }
    }
}
