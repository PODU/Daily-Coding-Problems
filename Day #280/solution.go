// Day 280: Detect cycle in undirected graph via Union-Find.
// An edge joining two already-connected vertices implies a cycle.
// Time O(V + E * alpha(V)), Space O(V).
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
		ru, rv := find(e[0]), find(e[1])
		if ru == rv {
			return true
		}
		parent[ru] = rv
	}
	return false
}

func main() {
	fmt.Println(hasCycle(4, [][2]int{{0, 1}, {1, 2}, {2, 0}, {2, 3}})) // true
	fmt.Println(hasCycle(4, [][2]int{{0, 1}, {1, 2}, {2, 3}}))          // false
}
