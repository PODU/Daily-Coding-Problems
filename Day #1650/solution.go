// Maximum spanning tree: Kruskal with edges sorted DESC by weight + Union-Find (path compression, union by rank). O(E log E).
package main

import (
	"fmt"
	"sort"
)

type Edge struct{ u, v, w int }

var parent, rnk []int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func unite(a, b int) bool {
	a, b = find(a), find(b)
	if a == b {
		return false
	}
	if rnk[a] < rnk[b] {
		a, b = b, a
	}
	parent[b] = a
	if rnk[a] == rnk[b] {
		rnk[a]++
	}
	return true
}

func maxSpanningTree(n int, edges []Edge) int {
	sort.Slice(edges, func(i, j int) bool { return edges[i].w > edges[j].w })
	parent = make([]int, n)
	rnk = make([]int, n)
	for i := 0; i < n; i++ {
		parent[i] = i
	}
	total := 0
	for _, e := range edges {
		if unite(e.u, e.v) {
			total += e.w
		}
	}
	return total
}

func main() {
	edges := []Edge{{0, 1, 1}, {0, 2, 2}, {0, 3, 3}, {1, 2, 4}, {2, 3, 5}}
	fmt.Printf("Maximum spanning tree weight: %d\n", maxSpanningTree(4, edges))
}
