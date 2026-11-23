//! Maximum-cardinality interval scheduling by earliest finishing time.
//!
//! Intervals are half-open. Sorting takes O(n log n) time; the scan takes
//! O(n) time and ordering/results use O(n) auxiliary space.
//! Choosing the earliest finish leaves at least as much room as any other
//! first choice, yielding the greedy exchange argument.

/// Returns original indices in deterministic finish-time order.
///
/// Invalid intervals (start greater than end) return None. Zero-length
/// intervals are allowed; consecutive intervals require next.start >= end.
pub fn max_non_overlapping(intervals: &[(i64, i64)]) -> Option<Vec<usize>> {
    if intervals.iter().any(|&(start, end)| start > end) {
        return None;
    }

    let mut order: Vec<usize> = (0..intervals.len()).collect();
    order.sort_unstable_by_key(|&index| {
        let (start, end) = intervals[index];
        (end, start, index)
    });

    let mut selected = Vec::new();
    let mut last_end: Option<i64> = None;
    for index in order {
        let (start, end) = intervals[index];
        if last_end.is_none_or(|previous| start >= previous) {
            selected.push(index);
            last_end = Some(end);
        }
    }
    Some(selected)
}

#[cfg(test)]
mod tests {
    use super::max_non_overlapping;

    #[test]
    fn empty_input() {
        assert_eq!(max_non_overlapping(&[]), Some(vec![]));
    }

    #[test]
    fn selects_earliest_finishing_intervals() {
        let intervals = [(0, 9), (1, 3), (3, 5), (4, 7), (5, 8)];
        assert_eq!(max_non_overlapping(&intervals), Some(vec![1, 2, 4]));
    }

    #[test]
    fn touching_endpoints_are_compatible() {
        assert_eq!(
            max_non_overlapping(&[(2, 4), (0, 2), (4, 6)]),
            Some(vec![1, 0, 2])
        );
    }

    #[test]
    fn handles_nested_duplicates_and_zero_length() {
        assert_eq!(
            max_non_overlapping(&[(0, 10), (2, 3), (2, 3), (3, 3), (3, 4)]),
            Some(vec![1, 3, 4])
        );
    }

    #[test]
    fn handles_negative_and_extreme_coordinates() {
        assert_eq!(
            max_non_overlapping(&[
                (i64::MIN, -5),
                (-5, 0),
                (0, i64::MAX),
                (i64::MIN, i64::MAX),
            ]),
            Some(vec![0, 1, 2])
        );
    }

    #[test]
    fn rejects_reversed_intervals() {
        assert_eq!(max_non_overlapping(&[(0, 1), (3, 2)]), None);
    }

    #[test]
    fn matches_brute_force_maximum_on_small_inputs() {
        let intervals = [(0, 4), (1, 2), (2, 3), (3, 5), (4, 6), (6, 6)];
        let selected = max_non_overlapping(&intervals).unwrap();
        let optimum = (0usize..(1 << intervals.len()))
            .filter(|mask| {
                let mut chosen: Vec<_> = (0..intervals.len())
                    .filter(|&index| mask & (1 << index) != 0)
                    .map(|index| intervals[index])
                    .collect();
                chosen.sort_unstable_by_key(|&(start, end)| (end, start));
                chosen.windows(2).all(|pair| pair[0].1 <= pair[1].0)
            })
            .map(|mask| mask.count_ones() as usize)
            .max()
            .unwrap();
        assert_eq!(selected.len(), optimum);
    }
}
