//! Disjoint-set union with path compression and union by size.

/// Tracks connectivity among indexed elements.
/// Construction and storage are O(n). Operations take amortized O(alpha(n)) time.
#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DisjointSet {
    /// Starts with len singleton components.
    pub fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
            size: vec![1; len],
            components: len,
        }
    }

    /// Returns the number of connected components.
    pub fn components(&self) -> usize {
        self.components
    }

    /// Returns the representative of an element, compressing its path.
    /// Returns None for an out-of-range index.
    pub fn find(&mut self, element: usize) -> Option<usize> {
        if element >= self.parent.len() {
            return None;
        }
        let mut root = element;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        let mut current = element;
        while self.parent[current] != root {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }
        Some(root)
    }

    /// Returns Some(true) for a new merge, Some(false) if already connected,
    /// or None if either index is invalid.
    pub fn union(&mut self, a: usize, b: usize) -> Option<bool> {
        let mut root_a = self.find(a)?;
        let mut root_b = self.find(b)?;
        if root_a == root_b {
            return Some(false);
        }
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
        self.components -= 1;
        Some(true)
    }

    /// Checks whether two valid indices belong to the same component.
    pub fn connected(&mut self, a: usize, b: usize) -> Option<bool> {
        Some(self.find(a)? == self.find(b)?)
    }

    /// Returns the size of the component containing element.
    pub fn component_size(&mut self, element: usize) -> Option<usize> {
        let root = self.find(element)?;
        Some(self.size[root])
    }
}

#[cfg(test)]
mod tests {
    use super::DisjointSet;

    #[test]
    fn singleton_components_and_empty_sets() {
        let mut empty = DisjointSet::new(0);
        assert_eq!(empty.components(), 0);
        assert_eq!(empty.find(0), None);
        let mut sets = DisjointSet::new(3);
        assert_eq!(sets.components(), 3);
        assert_eq!(sets.component_size(2), Some(1));
        assert_eq!(sets.connected(0, 1), Some(false));
    }

    #[test]
    fn unions_are_transitive_and_idempotent() {
        let mut sets = DisjointSet::new(5);
        assert_eq!(sets.union(0, 1), Some(true));
        assert_eq!(sets.union(2, 3), Some(true));
        assert_eq!(sets.union(1, 3), Some(true));
        assert_eq!(sets.union(0, 2), Some(false));
        assert_eq!(sets.connected(0, 3), Some(true));
        assert_eq!(sets.connected(0, 4), Some(false));
        assert_eq!(sets.component_size(2), Some(4));
        assert_eq!(sets.components(), 2);
    }

    #[test]
    fn invalid_indices_do_not_change_components() {
        let mut sets = DisjointSet::new(2);
        assert_eq!(sets.union(0, 2), None);
        assert_eq!(sets.connected(2, 0), None);
        assert_eq!(sets.find(usize::MAX), None);
        assert_eq!(sets.component_size(2), None);
        assert_eq!(sets.components(), 2);
    }

    #[test]
    fn compresses_paths_after_find() {
        let mut sets = DisjointSet::new(4);
        assert_eq!(sets.union(0, 1), Some(true));
        assert_eq!(sets.union(2, 3), Some(true));
        assert_eq!(sets.union(0, 2), Some(true));
        let root = sets.find(3).unwrap();
        assert_eq!(sets.parent[3], root);
        assert_eq!(sets.component_size(3), Some(4));
    }
}
