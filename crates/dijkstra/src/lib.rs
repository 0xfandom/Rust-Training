//! Single-source shortest paths for nonnegative weighted directed graphs.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Returns minimum distances from `start` to each vertex.
///
/// The graph uses adjacency lists of (destination, weight) pairs. Returns
/// `None` if the start or any edge endpoint is invalid. Unreachable vertices
/// have no distance. Paths whose total weight exceeds `u64::MAX` cannot be
/// represented and are ignored.
///
/// Time: O((V + E) log V). Auxiliary space: O(V + E) in the worst case.
pub fn dijkstra(graph: &[Vec<(usize, u64)>], start: usize) -> Option<Vec<Option<u64>>> {
    if start >= graph.len()
        || graph
            .iter()
            .flatten()
            .any(|&(neighbor, _)| neighbor >= graph.len())
    {
        return None;
    }

    let mut distances = vec![None; graph.len()];
    let mut pending = BinaryHeap::new();
    distances[start] = Some(0);
    pending.push(Reverse((0_u64, start)));

    while let Some(Reverse((distance, vertex))) = pending.pop() {
        if distances[vertex] != Some(distance) {
            continue;
        }

        for &(neighbor, weight) in &graph[vertex] {
            let Some(candidate) = distance.checked_add(weight) else {
                continue;
            };
            if distances[neighbor].is_none_or(|current| candidate < current) {
                distances[neighbor] = Some(candidate);
                pending.push(Reverse((candidate, neighbor)));
            }
        }
    }

    Some(distances)
}

#[cfg(test)]
mod tests {
    use super::dijkstra;

    #[test]
    fn chooses_cheaper_indirect_routes() {
        let graph = vec![vec![(1, 10), (2, 2)], vec![(3, 1)], vec![(1, 3), (3, 9)], vec![]];
        assert_eq!(dijkstra(&graph, 0), Some(vec![Some(0), Some(5), Some(2), Some(6)]));
    }

    #[test]
    fn handles_zero_weights_and_parallel_edges() {
        let graph = vec![vec![(1, 7), (1, 0)], vec![(2, 0)], vec![(0, 4)]];
        assert_eq!(dijkstra(&graph, 0), Some(vec![Some(0), Some(0), Some(0)]));
    }

    #[test]
    fn leaves_disconnected_vertices_unreachable() {
        let graph = vec![vec![(1, 2)], vec![], vec![(3, 1)], vec![]];
        assert_eq!(dijkstra(&graph, 0), Some(vec![Some(0), Some(2), None, None]));
    }

    #[test]
    fn rejects_invalid_graphs_and_starts() {
        assert_eq!(dijkstra(&[], 0), None);
        assert_eq!(dijkstra(&[vec![]], 1), None);
        assert_eq!(dijkstra(&[vec![(2, 1)], vec![]], 0), None);
        assert_eq!(dijkstra(&[vec![], vec![(9, 1)]], 0), None);
    }

    #[test]
    fn preserves_maximum_representable_distance() {
        let graph = vec![vec![(1, u64::MAX), (2, 1)], vec![(2, 1)], vec![]];
        assert_eq!(dijkstra(&graph, 0), Some(vec![Some(0), Some(u64::MAX), Some(1)]));
    }

    #[test]
    fn skips_paths_that_overflow() {
        let graph = vec![vec![(1, u64::MAX)], vec![(2, 1)], vec![]];
        assert_eq!(dijkstra(&graph, 0), Some(vec![Some(0), Some(u64::MAX), None]));
    }

    #[test]
    fn matches_bellman_ford_on_small_graphs() {
        let graph = vec![
            vec![(1, 4), (2, 9)],
            vec![(2, 2), (3, 7)],
            vec![(3, 1), (0, 8)],
            vec![(1, 0)],
            vec![],
        ];
        for start in 0..graph.len() {
            let mut expected = vec![None; graph.len()];
            expected[start] = Some(0_u64);
            for _ in 1..graph.len() {
                for (vertex, edges) in graph.iter().enumerate() {
                    if let Some(distance) = expected[vertex] {
                        for &(neighbor, weight) in edges {
                            if let Some(candidate) = distance.checked_add(weight) {
                                if expected[neighbor].is_none_or(|current| candidate < current) {
                                    expected[neighbor] = Some(candidate);
                                }
                            }
                        }
                    }
                }
            }
            assert_eq!(dijkstra(&graph, start), Some(expected));
        }
    }
}
