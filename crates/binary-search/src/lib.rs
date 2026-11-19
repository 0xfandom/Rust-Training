//! Binary search over ascending sorted integer slices.

/// Returns the index of a matching value, or `None` if absent.
///
/// The input must be sorted in ascending order. When duplicates exist,
/// any matching index may be returned.
///
/// Time: O(log n). Extra space: O(1).
pub fn binary_search(values: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = values.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match values[mid].cmp(&target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn empty_slice() {
        assert_eq!(binary_search(&[], 7), None);
    }

    #[test]
    fn one_element() {
        assert_eq!(binary_search(&[7], 7), Some(0));
        assert_eq!(binary_search(&[7], 8), None);
    }

    #[test]
    fn finds_boundaries_and_middle() {
        let numbers = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&numbers, 1), Some(0));
        assert_eq!(binary_search(&numbers, 5), Some(2));
        assert_eq!(binary_search(&numbers, 9), Some(4));
    }

    #[test]
    fn missing_values() {
        assert_eq!(binary_search(&[1, 3, 5, 7], 0), None);
        assert_eq!(binary_search(&[1, 3, 5, 7], 4), None);
        assert_eq!(binary_search(&[1, 3, 5, 7], 8), None);
    }

    #[test]
    fn duplicate_values() {
        let values = [1, 2, 2, 2, 3];
        let index = binary_search(&values, 2).expect("2 should be found");
        assert_eq!(values[index], 2);
    }

    #[test]
    fn extreme_values() {
        assert_eq!(binary_search(&[i32::MIN, 0, i32::MAX], i32::MAX), Some(2));
    }
}
