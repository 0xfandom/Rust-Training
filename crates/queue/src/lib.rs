//! A first-in, first-out queue backed by `VecDeque`.
//!
//! Enqueue takes amortized O(1) time, while dequeue and front take O(1).
//! Storage grows with the number of queued elements: O(n).

use std::collections::VecDeque;

/// A FIFO collection that owns its elements.
#[derive(Debug, Default)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Creates an empty queue without allocating.
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    /// Adds a value to the back of the queue.
    pub fn enqueue(&mut self, value: T) {
        self.items.push_back(value);
    }

    /// Removes and returns the oldest value, or `None` if empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// Borrows the oldest value without removing it.
    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    /// Returns the number of queued values.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn empty_queue_operations() {
        let mut queue = Queue::<i32>::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.front(), None);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn returns_values_in_fifo_order() {
        let mut queue = Queue::new();
        for value in [3, 1, 2] {
            queue.enqueue(value);
        }
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.front(), Some(&3));
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn supports_interleaved_operations() {
        let mut queue = Queue::new();
        queue.enqueue(10);
        assert_eq!(queue.dequeue(), Some(10));
        queue.enqueue(20);
        queue.enqueue(30);
        assert_eq!(queue.dequeue(), Some(20));
        queue.enqueue(40);
        assert_eq!(queue.front(), Some(&30));
        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.dequeue(), Some(40));
        assert!(queue.is_empty());
    }

    #[test]
    fn owns_non_copy_values() {
        let mut queue = Queue::new();
        queue.enqueue(String::from("first"));
        queue.enqueue(String::from("second"));
        assert_eq!(queue.front().map(String::as_str), Some("first"));
        assert_eq!(queue.dequeue(), Some(String::from("first")));
        assert_eq!(queue.dequeue(), Some(String::from("second")));
    }
}
