# Algorithm inventory

Version: 4. Update this file when an algorithm is proposed, merged, or retired. Compare behavior and aliases before opening an issue. Rust language exercises are tracked separately from algorithms.

| Area | Algorithm / variant | Implementation | Issue | PR | Status |
| --- | --- | --- | --- | --- | --- |
| Searching | Binary search (sorted integer slice) | crates/binary-search/src/lib.rs | #3 | #5 | merged |
| Sorting | Merge sort (stable integer slice) | crates/merge-sort/src/lib.rs | #6 | #7 | merged |
| Graph traversal | Breadth-first search (adjacency lists) | crates/graph-bfs/src/lib.rs | #9 | #10 | merged |
| Disjoint sets | Union-find (path compression, union by size) | crates/disjoint-set/src/lib.rs | #11 | #12 | merged |
| Graph traversal | Iterative depth-first search (adjacency lists) | crates/graph-dfs/src/lib.rs | #13 | #14 | merged |
| Priority queue | Binary min-heap (integer priorities) | crates/min-heap/src/lib.rs | #15 | #16 | merged |
| Graph ordering | Kahn topological sort (directed adjacency lists) | crates/graph-topological-sort/src/lib.rs | #17 | #18 | in review |

## Prioritized roadmap

Check inventory, open issues, branches, and PRs before selecting a candidate.

1. Core data structures: stack, queue, hash table, linked list, binary search tree, trie.
2. Graph algorithms: Dijkstra, Bellman-Ford, minimum spanning trees, strongly connected components.
3. Sorting/searching: insertion sort, quicksort, heap sort, distinct lower/upper bound contracts.
4. Arrays/strings: sliding window, two pointers, prefix sums, KMP, rolling hash.
5. Dynamic programming: knapsack, longest increasing subsequence, edit distance, longest common subsequence.
6. Greedy/backtracking: interval scheduling, Huffman coding, subsets, permutations, N-queens.
7. Number theory/bit manipulation: Euclidean GCD, modular exponentiation, sieve, bitset operations.
8. Geometry: orientation, segment intersection, convex hull.
9. Concurrency/performance: bounded work queue, parallel reductions, benchmarks and allocation analysis.

References: https://doc.rust-lang.org/std/collections/ ; https://doc.rust-lang.org/book/ ; https://cp-algorithms.com/ .
