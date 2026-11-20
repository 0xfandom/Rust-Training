//! Iterative depth-first traversal of an adjacency-list graph.

/// Returns reachable vertices in depth-first preorder.
///
/// Neighbors are explored in their adjacency-list order. Returns `None` if
/// `start` is outside the graph; invalid neighbor indices are ignored.
///
/// Time: O(V + E). Auxiliary space: O(V).
pub fn dfs(graph: &[Vec<usize>], start: usize) -> Option<Vec<usize>> {
    if start >= graph.len() {
        return None;
    }

    let mut visited = vec![false; graph.len()];
    let mut order = vec![start];
    // Each frame tracks the next neighbor to examine, like a recursive call.
    let mut stack = vec![(start, 0usize)];
    visited[start] = true;

    while let Some((vertex, next_neighbor)) = stack.last_mut() {
        if *next_neighbor == graph[*vertex].len() {
            stack.pop();
            continue;
        }

        let neighbor = graph[*vertex][*next_neighbor];
        *next_neighbor += 1;
        if neighbor < graph.len() && !visited[neighbor] {
            visited[neighbor] = true;
            order.push(neighbor);
            stack.push((neighbor, 0));
        }
    }
    Some(order)
}

#[cfg(test)]
mod tests {
    use super::dfs;

    #[test]
    fn follows_depth_first_neighbor_order() {
        let graph = vec![vec![1, 2], vec![3, 4], vec![5], vec![], vec![], vec![]];
        assert_eq!(dfs(&graph, 0), Some(vec![0, 1, 3, 4, 2, 5]));
    }

    #[test]
    fn handles_cycles_self_loops_and_duplicate_edges() {
        let graph = vec![vec![0, 1, 1], vec![2, 0], vec![1]];
        assert_eq!(dfs(&graph, 0), Some(vec![0, 1, 2]));
    }

    #[test]
    fn only_visits_reachable_vertices() {
        let graph = vec![vec![1], vec![], vec![3], vec![]];
        assert_eq!(dfs(&graph, 2), Some(vec![2, 3]));
    }

    #[test]
    fn handles_invalid_indices_and_empty_graphs() {
        assert_eq!(dfs(&[], 0), None);
        assert_eq!(dfs(&[vec![]], 1), None);
        assert_eq!(dfs(&[vec![99, 1], vec![]], 0), Some(vec![0, 1]));
    }

    #[test]
    fn traverses_deep_paths_without_recursion() {
        let count = 10_000;
        let mut graph = vec![Vec::new(); count];
        for (index, neighbors) in graph.iter_mut().enumerate().take(count - 1) {
            neighbors.push(index + 1);
        }
        let order = dfs(&graph, 0).unwrap();
        assert_eq!(order.len(), count);
        assert_eq!(order[0], 0);
        assert_eq!(order[count - 1], count - 1);
    }
}
