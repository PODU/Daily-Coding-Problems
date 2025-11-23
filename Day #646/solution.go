// Day 646: Count friend groups = connected components in an undirected graph.
// Approach: Union-Find (DSU) with path compression + union by rank.
// Time: O(V + E * alpha(V)), Space: O(V).
package main

import "fmt"

var parent, rank_ []int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func unite(a, b int) {
	a, b = find(a), find(b)
	if a == b {
		return
	}
	if rank_[a] < rank_[b] {
		a, b = b, a
	}
	parent[b] = a
	if rank_[a] == rank_[b] {
		rank_[a]++
	}
}

func countGroups(adj map[int][]int) int {
	n := len(adj)
	parent = make([]int, n)
	rank_ = make([]int, n)
	for i := 0; i < n; i++ {
		parent[i] = i
	}
	for u, nbrs := range adj {
		for _, v := range nbrs {
			unite(u, v)
		}
	}
	roots := map[int]bool{}
	for u := range adj {
		roots[find(u)] = true
	}
	return len(roots)
}

func main() {
	adj := map[int][]int{
		0: {1, 2}, 1: {0, 5}, 2: {0}, 3: {6}, 4: {}, 5: {1}, 6: {3},
	}
	fmt.Println(countGroups(adj)) // 3
}
