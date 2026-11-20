//! An array-backed binary min-heap for integer priorities.

/// A priority queue that removes the smallest value first.
///
/// `push` and `pop` take O(log n) time; `peek` takes O(1).
/// Storage is O(n), and duplicate priorities are retained.
#[derive(Debug, Default)]
pub struct MinHeap {
    values: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn peek(&self) -> Option<i32> {
        self.values.first().copied()
    }

    pub fn push(&mut self, value: i32) {
        self.values.push(value);
        let mut index = self.values.len() - 1;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.values[parent] <= self.values[index] {
                break;
            }
            self.values.swap(parent, index);
            index = parent;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result = self.values.pop()?;
        if self.values.is_empty() {
            return Some(result);
        }
        let smallest = std::mem::replace(&mut self.values[0], result);
        let mut index: usize = 0;
        let len = self.values.len();
        loop {
            let Some(left) = index.checked_mul(2).and_then(|n| n.checked_add(1)) else {
                break;
            };
            if left >= len {
                break;
            }
            let right = left + 1;
            let child = if right < len && self.values[right] < self.values[left] {
                right
            } else {
                left
            };
            if self.values[index] <= self.values[child] {
                break;
            }
            self.values.swap(index, child);
            index = child;
        }
        Some(smallest)
    }
}

#[cfg(test)]
mod tests {
    use super::MinHeap;

    #[test]
    fn empty_queue() {
        let mut heap = MinHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn singleton_and_duplicates() {
        let mut heap = MinHeap::new();
        heap.push(4);
        heap.push(4);
        assert_eq!(heap.peek(), Some(4));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn pops_values_in_sorted_order() {
        let mut heap = MinHeap::new();
        for value in [9, -3, 5, i32::MAX, i32::MIN, 0, 5] {
            heap.push(value);
        }
        let mut result = Vec::new();
        while let Some(value) = heap.pop() {
            result.push(value);
        }
        assert_eq!(result, vec![i32::MIN, -3, 0, 5, 5, 9, i32::MAX]);
    }

    #[test]
    fn handles_interleaved_operations() {
        let mut heap = MinHeap::new();
        heap.push(8);
        heap.push(3);
        assert_eq!(heap.pop(), Some(3));
        heap.push(1);
        heap.push(6);
        assert_eq!(heap.peek(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), Some(8));
        assert!(heap.is_empty());
    }
}
