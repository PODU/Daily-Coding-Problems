// Count connected components (friend groups) via Union-Find.
// Time O(V+E a(V)), Space O(V).
package main

import "fmt"

var parent map[int]int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func unite(a, b int) { parent[find(a)] = find(b) }

func countGroups(adj map[int][]int) int {
	parent = make(map[int]int)
	for k := range adj {
		parent[k] = k
	}
	for u, nbrs := range adj {
		for _, v := range nbrs {
			unite(u, v)
		}
	}
	roots := make(map[int]bool)
	for k := range adj {
		roots[find(k)] = true
	}
	return len(roots)
}

func main() {
	adj := map[int][]int{0: {1, 2}, 1: {0, 5}, 2: {0}, 3: {6}, 4: {}, 5: {1}, 6: {3}}
	fmt.Println(countGroups(adj))
}
