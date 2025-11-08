// Minimally-connected = tree: Union-Find detects cycle on union; final check connected && edges==V-1. Time O(E a(V)), Space O(V).
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

func isMinimallyConnected(V int, edges [][2]int) bool {
	if len(edges) != V-1 {
		return false
	}
	parent = make([]int, V)
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
	root := find(0)
	for i := 1; i < V; i++ {
		if find(i) != root {
			return false
		}
	}
	return true
}

func main() {
	if isMinimallyConnected(4, [][2]int{{0, 1}, {1, 2}, {2, 3}}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
	if isMinimallyConnected(4, [][2]int{{0, 1}, {1, 2}, {2, 3}, {3, 0}}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
