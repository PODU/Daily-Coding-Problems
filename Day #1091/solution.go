// Count connected components via Union-Find with path compression. Time ~O(N+E*alpha), Space O(N).
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
	for u := range adj {
		parent[u] = u
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
		0: {1, 2}, 1: {0, 5}, 2: {0}, 3: {6}, 4: {}, 5: {1}, 6: {3}}
	fmt.Println(countGroups(adj))
}
