//! A last-in, first-out stack backed by a growable vector.
//!
//! Push takes amortized O(1) time (occasional growth reallocates and copies
//! existing elements). Pop and peek take O(1) time; storage is O(n).

/// A last-in, first-out collection that owns its elements.
#[derive(Debug, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates an empty stack without allocating.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Adds a value to the top of the stack.
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Removes and returns the top value, or `None` if empty.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Borrows the top value without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Returns the number of values in the stack.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn empty_stack_operations() {
        let mut stack = Stack::<i32>::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn returns_values_in_lifo_order() {
        let mut stack = Stack::new();
        for value in [1, 2, 3] {
            stack.push(value);
        }
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn supports_interleaved_operations() {
        let mut stack = Stack::new();
        stack.push(10);
        assert_eq!(stack.pop(), Some(10));
        stack.push(20);
        stack.push(30);
        assert_eq!(stack.pop(), Some(30));
        stack.push(40);
        assert_eq!(stack.pop(), Some(40));
        assert_eq!(stack.pop(), Some(20));
        assert!(stack.is_empty());
    }

    #[test]
    fn owns_non_copy_values() {
        let mut stack = Stack::new();
        stack.push(String::from("first"));
        stack.push(String::from("second"));
        assert_eq!(stack.peek().map(String::as_str), Some("second"));
        assert_eq!(stack.pop(), Some(String::from("second")));
        assert_eq!(stack.pop(), Some(String::from("first")));
    }
}
