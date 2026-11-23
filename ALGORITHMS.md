# Algorithm inventory

Version: 16. Update this file when an algorithm is proposed, merged, or retired. Compare behavior and aliases before opening an issue. Rust language exercises are tracked separately from algorithms.

| Area | Algorithm / variant | Implementation | Issue | PR | Status |
| --- | --- | --- | --- | --- | --- |
| Searching | Binary search (sorted integer slice) | crates/binary-search/src/lib.rs | #3 | #5 | merged |
| Sorting | Merge sort (stable integer slice) | crates/merge-sort/src/lib.rs | #6 | #7 | merged |
| Graph traversal | Breadth-first search (adjacency lists) | crates/graph-bfs/src/lib.rs | #9 | #10 | merged |
| Disjoint sets | Union-find (path compression, union by size) | crates/disjoint-set/src/lib.rs | #11 | #12 | merged |
| Graph traversal | Iterative depth-first search (adjacency lists) | crates/graph-dfs/src/lib.rs | #13 | #14 | merged |
| Priority queue | Binary min-heap (integer priorities) | crates/min-heap/src/lib.rs | #15 | #16 | merged |
| Graph ordering | Kahn topological sort (directed adjacency lists) | crates/graph-topological-sort/src/lib.rs | #17 | #18 | merged |
| Number theory | Euclidean GCD (iterative, u64) | crates/euclidean-gcd/src/lib.rs | #19 | #20 | merged |
| Data structures | Generic LIFO stack (Vec-backed) | crates/stack/src/lib.rs | #21 | #22 | merged |
| Data structures | Generic FIFO queue (VecDeque-backed) | crates/queue/src/lib.rs | #23 | #24 | merged |
| Data structures | Singly linked list (owned boxed nodes, head operations) | crates/singly-linked-list/src/lib.rs | #25 | #26 | merged |
| Sorting | Stable insertion sort (in-place integer slice) | crates/insertion-sort/src/lib.rs | #27 | #28 | merged |
| Strings / data structures | Lowercase ASCII prefix trie (exact and prefix lookup) | crates/trie/src/lib.rs | #29 | #30 | merged |
| Arrays | Prefix sums (i64 input, i128 half-open range queries) | crates/prefix-sums/src/lib.rs | #31 | #32 | merged |

| Number theory | Modular exponentiation (binary exponentiation, u64) | crates/modular-exponentiation/src/lib.rs | #33 | #34 | merged |
| Number theory | Sieve of Eratosthenes (inclusive prime generation) | crates/sieve-of-eratosthenes/src/lib.rs | #35 | #36 | merged |

| Arrays | Sliding window maximum (monotonic deque, i64) | crates/sliding-window-maximum/src/lib.rs | #37 | #38 | merged |
| Greedy | Interval scheduling (earliest finish, half-open i64) | crates/interval-scheduling/src/lib.rs | #39 | #40 | merged |
| Shortest paths | Dijkstra (nonnegative u64 edge weights) | crates/dijkstra/src/lib.rs | #41 | pending | in review |

## Prioritized roadmap

Check inventory, open issues, branches, and PRs before selecting a candidate.

1. Core data structures: hash table, binary search tree.
2. Graph algorithms: Dijkstra, Bellman-Ford, minimum spanning trees, strongly connected components.
3. Sorting/searching: quicksort, heap sort, distinct lower/upper bound contracts.
4. Arrays/strings: sliding window, two pointers, KMP, rolling hash.
5. Dynamic programming: knapsack, longest increasing subsequence, edit distance, longest common subsequence.
6. Greedy/backtracking: interval scheduling, Huffman coding, subsets, permutations, N-queens.
7. Number theory/bit manipulation: prime factorization, segmented sieve, bitset operations.
8. Geometry: orientation, segment intersection, convex hull.
9. Concurrency/performance: bounded work queue, parallel reductions, benchmarks and allocation analysis.

References: https://doc.rust-lang.org/std/collections/ ; https://doc.rust-lang.org/book/ ; https://cp-algorithms.com/ .
