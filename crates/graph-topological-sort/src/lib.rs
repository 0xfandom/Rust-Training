//! Topological ordering of directed adjacency-list graphs using Kahn's algorithm.

use std::collections::VecDeque;

/// Returns a topological ordering, or `None` for a cycle or invalid edge.
///
/// Vertices are numbered `0..graph.len()`. Initially available vertices are
/// queued in ascending order; newly available vertices are queued in adjacency
/// list order. This makes the result deterministic for a given input.
///
/// Duplicate edges count as separate edges. The algorithm runs in O(V + E)
/// time and uses O(V) auxiliary space.
pub fn topological_sort(graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut indegree = vec![0usize; graph.len()];
    for neighbors in graph {
        for &neighbor in neighbors {
            let degree = indegree.get_mut(neighbor)?;
            *degree = degree.checked_add(1)?;
        }
    }

    let mut ready: VecDeque<usize> = indegree
        .iter()
        .enumerate()
        .filter_map(|(vertex, &degree)| (degree == 0).then_some(vertex))
        .collect();
    let mut order = Vec::with_capacity(graph.len());

    while let Some(vertex) = ready.pop_front() {
        order.push(vertex);
        for &neighbor in &graph[vertex] {
            indegree[neighbor] -= 1;
            if indegree[neighbor] == 0 {
                ready.push_back(neighbor);
            }
        }
    }

    (order.len() == graph.len()).then_some(order)
}

#[cfg(test)]
mod tests {
    use super::topological_sort;

    #[test]
    fn empty_graph() {
        assert_eq!(topological_sort(&[]), Some(vec![]));
    }

    #[test]
    fn directed_acyclic_graph() {
        let graph = vec![vec![2], vec![2, 3], vec![3], vec![]];
        assert_eq!(topological_sort(&graph), Some(vec![0, 1, 2, 3]));
    }

    #[test]
    fn disconnected_and_isolated_vertices() {
        let graph = vec![vec![1], vec![], vec![], vec![4], vec![]];
        assert_eq!(topological_sort(&graph), Some(vec![0, 2, 3, 1, 4]));
    }

    #[test]
    fn duplicate_edges() {
        let graph = vec![vec![1, 1], vec![2], vec![]];
        assert_eq!(topological_sort(&graph), Some(vec![0, 1, 2]));
    }

    #[test]
    fn cycle() {
        let graph = vec![vec![1], vec![2], vec![0]];
        assert_eq!(topological_sort(&graph), None);
    }

    #[test]
    fn self_loop() {
        assert_eq!(topological_sort(&[vec![0]]), None);
    }

    #[test]
    fn invalid_vertex() {
        assert_eq!(topological_sort(&[vec![1]]), None);
    }
}
