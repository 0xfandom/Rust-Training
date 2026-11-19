//! Breadth-first traversal of an adjacency-list graph.

use std::collections::VecDeque;

/// Returns vertices reachable from `start` in breadth-first order.
///
/// Neighbor order follows each adjacency list. Returns `None` if `start`
/// is outside the graph. Invalid neighbor indices are ignored.
///
/// Time: O(V + E). Auxiliary space: O(V).
pub fn bfs(graph: &[Vec<usize>], start: usize) -> Option<Vec<usize>> {
    if start >= graph.len() {
        return None;
    }

    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(vertex) = queue.pop_front() {
        order.push(vertex);
        for &neighbor in &graph[vertex] {
            if neighbor < graph.len() && !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    Some(order)
}

#[cfg(test)]
mod tests {
    use super::bfs;

    #[test]
    fn visits_vertices_by_distance() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        assert_eq!(bfs(&graph, 0), Some(vec![0, 1, 2, 3]));
    }

    #[test]
    fn handles_cycles_and_repeated_edges() {
        let graph = vec![vec![1, 1], vec![0, 2], vec![1]];
        assert_eq!(bfs(&graph, 0), Some(vec![0, 1, 2]));
    }

    #[test]
    fn excludes_disconnected_vertices() {
        let graph = vec![vec![1], vec![], vec![3], vec![]];
        assert_eq!(bfs(&graph, 2), Some(vec![2, 3]));
    }

    #[test]
    fn handles_empty_and_invalid_starts() {
        assert_eq!(bfs(&[], 0), None);
        assert_eq!(bfs(&[vec![]], 1), None);
        assert_eq!(bfs(&[vec![]], 0), Some(vec![0]));
    }

    #[test]
    fn ignores_invalid_neighbor_indices() {
        assert_eq!(bfs(&[vec![3, 1], vec![]], 0), Some(vec![0, 1]));
    }
}
