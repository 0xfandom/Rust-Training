//! A prefix trie for lowercase ASCII words.
//!
//! All operations take O(L) time, where L is the input byte length.
//! Each node reserves 26 child slots, so space grows with the number
//! of distinct prefixes.

#[derive(Default)]
struct Node {
    terminal: bool,
    children: [Option<Box<Node>>; 26],
}

/// An owned trie containing words made only of ASCII `a` through `z`.
///
/// Empty strings are accepted as words and prefixes. Inputs containing any
/// other byte are rejected; invalid insertion never changes the trie.
#[derive(Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    /// Creates an empty trie.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a word, returning `true` only when it was not already present.
    ///
    /// Invalid inputs return `false` without changing the trie.
    pub fn insert(&mut self, word: &str) -> bool {
        if !valid(word) {
            return false;
        }

        let mut node = &mut self.root;
        for byte in word.bytes() {
            node = node.children[(byte - b'a') as usize]
                .get_or_insert_with(|| Box::new(Node::default()));
        }
        let was_present = node.terminal;
        node.terminal = true;
        !was_present
    }

    /// Returns whether an exact word is present.
    pub fn contains(&self, word: &str) -> bool {
        self.find_node(word).is_some_and(|node| node.terminal)
    }

    /// Returns whether any stored word begins with the given prefix.
    ///
    /// The empty prefix always matches, including in an empty trie.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    fn find_node(&self, text: &str) -> Option<&Node> {
        if !valid(text) {
            return None;
        }

        let mut node = &self.root;
        for byte in text.bytes() {
            node = node.children[(byte - b'a') as usize].as_deref()?;
        }
        Some(node)
    }
}

fn valid(text: &str) -> bool {
    text.bytes().all(|byte| byte.is_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn shared_prefixes_and_exact_words() {
        let mut trie = Trie::new();
        assert!(trie.insert("car"));
        assert!(trie.insert("cart"));
        assert!(trie.insert("cat"));
        assert!(trie.contains("car"));
        assert!(trie.contains("cart"));
        assert!(trie.contains("cat"));
        assert!(!trie.contains("ca"));
        assert!(trie.starts_with("ca"));
        assert!(!trie.starts_with("cab"));
    }

    #[test]
    fn duplicate_insert_does_not_add_a_word() {
        let mut trie = Trie::new();
        assert!(trie.insert("rust"));
        assert!(!trie.insert("rust"));
        assert!(trie.contains("rust"));
    }

    #[test]
    fn empty_word_and_prefix() {
        let mut trie = Trie::new();
        assert!(trie.starts_with(""));
        assert!(!trie.contains(""));
        assert!(trie.insert(""));
        assert!(!trie.insert(""));
        assert!(trie.contains(""));
        assert!(!trie.contains("a"));
    }

    #[test]
    fn invalid_input_does_not_create_prefixes() {
        let mut trie = Trie::new();
        for word in ["abC", "ab!", "abé", "ab ", "A"] {
            assert!(!trie.insert(word));
            assert!(!trie.contains(word));
            assert!(!trie.starts_with(word));
        }
        assert!(!trie.starts_with("ab"));
        assert!(!trie.contains("ab"));
        assert!(trie.insert("abc"));
        assert!(!trie.insert("abc!"));
        assert!(trie.contains("abc"));
        assert!(!trie.contains("abc!"));
    }

    #[test]
    fn missing_words_and_prefixes() {
        let mut trie = Trie::new();
        assert!(!trie.contains("rust"));
        assert!(!trie.starts_with("r"));
        assert!(trie.insert("rust"));
        assert!(!trie.contains("rusty"));
        assert!(!trie.starts_with("ruby"));
        assert!(trie.starts_with("rus"));
    }
}
