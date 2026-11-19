//! Ownership and move semantics exercises.

/// Takes ownership of a string and returns its length and the original value.
pub fn measure_owned(value: String) -> (usize, String) {
    (value.len(), value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_a_string_preserves_its_contents() {
        let original = String::from("rust");
        let (length, recovered) = measure_owned(original);
        assert_eq!(length, 4);
        assert_eq!(recovered, "rust");
    }
}
