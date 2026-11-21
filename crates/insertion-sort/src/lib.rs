//! In-place insertion sort for signed integers.
//!
//! Stable: equal values are never shifted past each other.
//! Worst-case and average time: O(n²); already-sorted time: O(n).
//! Auxiliary space: O(1). No heap allocations.

/// Sorts the slice in ascending order using insertion sort.
///
/// The prefix before `i` is sorted at the start of each iteration.
pub fn insertion_sort(values: &mut [i32]) {
    for i in 1..values.len() {
        let current = values[i];
        let mut j = i;
        while j > 0 && values[j - 1] > current {
            values[j] = values[j - 1];
            j -= 1;
        }
        values[j] = current;
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn empty_and_singleton() {
        let mut empty: [i32; 0] = [];
        insertion_sort(&mut empty);
        assert_eq!(empty, []);
        let mut one = [42];
        insertion_sort(&mut one);
        assert_eq!(one, [42]);
    }

    #[test]
    fn sorted_and_reverse_sorted() {
        let mut sorted = [1, 2, 3, 4];
        insertion_sort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3, 4]);
        let mut reversed = [4, 3, 2, 1];
        insertion_sort(&mut reversed);
        assert_eq!(reversed, [1, 2, 3, 4]);
    }

    #[test]
    fn duplicates_and_negative_numbers() {
        let mut values = [3, -1, 3, 0, -1, 2];
        insertion_sort(&mut values);
        assert_eq!(values, [-1, -1, 0, 2, 3, 3]);
    }

    #[test]
    fn integer_boundaries() {
        let mut values = [i32::MAX, 0, i32::MIN, i32::MAX];
        insertion_sort(&mut values);
        assert_eq!(values, [i32::MIN, 0, i32::MAX, i32::MAX]);
    }
}
