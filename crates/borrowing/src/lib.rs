//! Borrowing exercises.

/// Counts bytes without taking ownership of the input.
pub fn byte_count(value: &str) -> usize {
    value.len()
}

/// Adds a suffix using an exclusive mutable borrow.
pub fn append_suffix(value: &mut String, suffix: &str) {
    value.push_str(suffix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_borrow_does_not_consume() {
        let value = String::from("hello");
        assert_eq!(byte_count(&value), 5);
        assert_eq!(value, "hello");
    }

    #[test]
    fn byte_count_uses_utf8_bytes() {
        assert_eq!(byte_count("é"), 2);
    }

    #[test]
    fn mutable_borrow_updates_in_place() {
        let mut value = String::from("hello");
        append_suffix(&mut value, " rust");
        assert_eq!(value, "hello rust");
    }
}
