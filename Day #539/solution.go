// Day 539: Detect a cycle in an undirected graph using union-find (disjoint set).
// For each edge, if endpoints already share a root -> cycle. Time O(E*alpha(V)), Space O(V).
package main

import "fmt"

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
		return false // already connected -> cycle
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

func hasCycle(n int, edges [][2]int) bool {
	parent = make([]int, n)
	rnk = make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	for _, e := range edges {
		if !unite(e[0], e[1]) {
			return true
		}
	}
	return false
}

func main() {
	cyclic := [][2]int{{0, 1}, {1, 2}, {2, 3}, {3, 0}}
	tree := [][2]int{{0, 1}, {1, 2}, {2, 3}}
	fmt.Println(hasCycle(4, cyclic))
	fmt.Println(hasCycle(4, tree))
}
