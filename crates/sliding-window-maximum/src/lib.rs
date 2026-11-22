//! Sliding-window maxima using a monotonic deque.
//!
//! Each index enters and leaves the deque at most once, giving O(n) time.
//! The deque stores at most `window` indices, using O(window) extra space.

use std::collections::VecDeque;

/// Returns the maximum of every contiguous window, from left to right.
///
/// A zero-sized window or a window larger than the input yields no results.
/// Equal values retain only the most recent index; the earlier equal value
/// cannot outlive it in any subsequent window.
pub fn sliding_window_max(values: &[i64], window: usize) -> Vec<i64> {
    if window == 0 || window > values.len() {
        return Vec::new();
    }

    let mut candidates: VecDeque<usize> = VecDeque::new();
    let mut maxima = Vec::with_capacity(values.len() - window + 1);

    for (index, &value) in values.iter().enumerate() {
        // An index is expired once its distance from the current index
        // reaches the window width.
        while candidates
            .front()
            .is_some_and(|&front| index - front >= window)
        {
            candidates.pop_front();
        }

        while candidates
            .back()
            .is_some_and(|&back| values[back] <= value)
        {
            candidates.pop_back();
        }

        candidates.push_back(index);

        if index + 1 >= window {
            maxima.push(values[*candidates.front().expect("nonempty candidate deque")]);
        }
    }

    maxima
}

#[cfg(test)]
mod tests {
    use super::sliding_window_max;

    #[test]
    fn rejects_invalid_window_sizes() {
        assert!(sliding_window_max(&[], 1).is_empty());
        assert!(sliding_window_max(&[1, 2], 0).is_empty());
        assert!(sliding_window_max(&[1, 2], 3).is_empty());
    }

    #[test]
    fn handles_single_and_full_windows() {
        assert_eq!(sliding_window_max(&[4, -2, 7], 1), vec![4, -2, 7]);
        assert_eq!(sliding_window_max(&[4, -2, 7], 3), vec![7]);
    }

    #[test]
    fn handles_increasing_decreasing_and_duplicates() {
        assert_eq!(sliding_window_max(&[1, 2, 3, 4], 2), vec![2, 3, 4]);
        assert_eq!(sliding_window_max(&[4, 3, 2, 1], 2), vec![4, 3, 2]);
        assert_eq!(sliding_window_max(&[5, 5, 5, 5], 3), vec![5, 5]);
    }

    #[test]
    fn handles_negative_and_integer_limits() {
        assert_eq!(
            sliding_window_max(&[i64::MIN, -1, i64::MAX, i64::MIN], 2),
            vec![-1, i64::MAX, i64::MAX]
        );
    }

    #[test]
    fn matches_reference_for_small_inputs() {
        let values = [3, -1, 3, 0, 8, 8, -5, 4];
        for width in 1..=values.len() {
            let expected: Vec<i64> = values
                .windows(width)
                .map(|part| *part.iter().max().unwrap())
                .collect();
            assert_eq!(sliding_window_max(&values, width), expected);
        }
    }
}
