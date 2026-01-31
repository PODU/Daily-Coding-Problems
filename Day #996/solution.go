// Day 996: Maximum weight spanning tree.
// Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// O(E log E) time, O(V) space.
package main

import (
	"fmt"
	"sort"
)

var parent []int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func union(a, b int) bool {
	ra, rb := find(a), find(b)
	if ra == rb {
		return false
	}
	parent[ra] = rb
	return true
}

func main() {
	n := 4
	// {weight, u, v}
	edges := [][3]int{{1, 0, 1}, {3, 0, 2}, {2, 1, 2}, {4, 1, 3}, {5, 2, 3}}
	sort.Slice(edges, func(i, j int) bool { return edges[i][0] > edges[j][0] }) // heaviest first
	parent = make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	total := 0
	for _, e := range edges {
		if union(e[1], e[2]) {
			total += e[0]
		}
	}
	fmt.Println("Maximum spanning tree weight:", total) // 12
}
