// Day 1675: Detect a cycle in an undirected graph via Union-Find.
// Union endpoints; a cycle exists if an edge joins already-connected nodes.
// Time O(E*alpha(V)), Space O(V).
package main

import "fmt"

func hasCycle(n int, edges [][2]int) bool {
	parent := make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	var find func(int) int
	find = func(x int) int {
		for parent[x] != x {
			parent[x] = parent[parent[x]]
			x = parent[x]
		}
		return x
	}
	for _, e := range edges {
		ra, rb := find(e[0]), find(e[1])
		if ra == rb {
			return true
		}
		parent[ra] = rb
	}
	return false
}

func main() {
	fmt.Println(hasCycle(4, [][2]int{{0, 1}, {1, 2}, {2, 3}, {3, 0}})) // true
	fmt.Println(hasCycle(4, [][2]int{{0, 1}, {1, 2}, {2, 3}}))         // false
}
