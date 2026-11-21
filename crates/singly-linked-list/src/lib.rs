//! A singly linked list with heap-allocated, owned nodes.
//!
//! Inserting or removing the head takes O(1) time and uses no element copies.
//! Each insertion allocates one node; n elements use O(n) space.
//! The list owns its head, and each node owns the remainder of the chain.

/// A singly linked list that owns its elements.
#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    /// Creates an empty list without allocating.
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// Inserts a value at the head in O(1) time.
    pub fn push_front(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
        self.len += 1;
    }

    /// Removes and returns the head value in O(1) time.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let Node { value, next } = *node;
            self.head = next;
            self.len -= 1;
            value
        })
    }

    /// Borrows the head value without removing it.
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        // Detach each successor so dropping a long list does not recurse.
        while let Some(mut node) = self.head.take() {
            self.head = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    #[test]
    fn empty_list_operations() {
        let mut list = SinglyLinkedList::<i32>::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn returns_values_in_head_order() {
        let mut list = SinglyLinkedList::new();
        for value in [1, 2, 3] {
            list.push_front(value);
        }
        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&3));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn maintains_length_across_interleaved_operations() {
        let mut list = SinglyLinkedList::new();
        list.push_front(10);
        assert_eq!(list.pop_front(), Some(10));
        list.push_front(20);
        list.push_front(30);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(30));
        list.push_front(40);
        assert_eq!(list.front(), Some(&40));
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn owns_non_copy_values() {
        let mut list = SinglyLinkedList::new();
        list.push_front(String::from("first"));
        list.push_front(String::from("second"));
        assert_eq!(list.front().map(String::as_str), Some("second"));
        assert_eq!(list.pop_front().as_deref(), Some("second"));
        assert_eq!(list.pop_front().as_deref(), Some("first"));
    }

    #[test]
    fn drops_long_lists_without_recursive_descent() {
        let mut list = SinglyLinkedList::new();
        for value in 0..20_000 {
            list.push_front(value);
        }
        assert_eq!(list.len(), 20_000);
    }
}
