//! Stable merge sort for integer slices.

/// Sorts the slice in ascending order using merge sort.
///
/// Equal values are taken from the left half first, keeping merge order stable.
/// Time: O(n log n). Extra space: O(n).
pub fn merge_sort(values: &mut [i32]) {
    let mut buffer = values.to_vec();
    sort_range(values, &mut buffer);
}

fn sort_range(values: &mut [i32], buffer: &mut [i32]) {
    if values.len() < 2 {
        return;
    }
    let middle = values.len() / 2;
    let (left, right) = values.split_at_mut(middle);
    let (left_buffer, right_buffer) = buffer.split_at_mut(middle);
    sort_range(left, left_buffer);
    sort_range(right, right_buffer);

    let (mut i, mut j) = (0, middle);
    for slot in buffer.iter_mut() {
        if i < middle && (j == values.len() || values[i] <= values[j]) {
            *slot = values[i];
            i += 1;
        } else {
            *slot = values[j];
            j += 1;
        }
    }
    values.copy_from_slice(buffer);
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn handles_empty_and_singleton() {
        let mut empty: [i32; 0] = [];
        merge_sort(&mut empty);
        assert_eq!(empty, []);
        let mut one = [7];
        merge_sort(&mut one);
        assert_eq!(one, [7]);
    }

    #[test]
    fn sorts_unsorted_and_reverse_sorted() {
        let mut values = [8, 3, 1, 9, 2];
        merge_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 8, 9]);
        let mut reverse = [5, 4, 3, 2, 1];
        merge_sort(&mut reverse);
        assert_eq!(reverse, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicates_and_negative_numbers() {
        let mut values = [0, -2, 3, -2, 3, i32::MIN, i32::MAX];
        merge_sort(&mut values);
        assert_eq!(values, [i32::MIN, -2, -2, 0, 3, 3, i32::MAX]);
    }

    #[test]
    fn preserves_sorted_input() {
        let mut values = [-4, 0, 1, 2, 9];
        merge_sort(&mut values);
        assert_eq!(values, [-4, 0, 1, 2, 9]);
    }
}
