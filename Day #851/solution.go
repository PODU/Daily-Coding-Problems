// Day 851: a graph is minimally-connected iff it is a tree: connected AND edges == nodes-1.
// Union-Find detects cycles and connectivity in O(E alpha(V)).
package main

import "fmt"

var parent []int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func isMinimallyConnected(n int, edges [][2]int) bool {
	if len(edges) != n-1 {
		return false
	}
	parent = make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	for _, e := range edges {
		ra, rb := find(e[0]), find(e[1])
		if ra == rb {
			return false // cycle
		}
		parent[ra] = rb
	}
	roots := map[int]bool{}
	for i := 0; i < n; i++ {
		roots[find(i)] = true
	}
	return len(roots) == 1
}

func main() {
	fmt.Println(isMinimallyConnected(5, [][2]int{{0, 1}, {0, 2}, {1, 3}, {1, 4}})) // true
	fmt.Println(isMinimallyConnected(4, [][2]int{{0, 1}, {1, 2}, {2, 0}, {2, 3}})) // false
}
