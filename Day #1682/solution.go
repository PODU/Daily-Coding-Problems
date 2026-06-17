// Day 1682: Graph k-colorability via backtracking with pruning.
// Time O(k^V) worst case, Space O(V).
package main

import "fmt"

func canColor(adj [][]int, k int) bool {
	n := len(adj)
	color := make([]int, n)
	for i := range color {
		color[i] = -1
	}
	var solve func(int) bool
	solve = func(v int) bool {
		if v == n {
			return true
		}
		for c := 0; c < k; c++ {
			ok := true
			for u := 0; u < n; u++ {
				if adj[v][u] == 1 && color[u] == c {
					ok = false
					break
				}
			}
			if ok {
				color[v] = c
				if solve(v + 1) {
					return true
				}
				color[v] = -1
			}
		}
		return false
	}
	return solve(0)
}

func main() {
	tri := [][]int{{0, 1, 1}, {1, 0, 1}, {1, 1, 0}}
	fmt.Println(canColor(tri, 2)) // false
	fmt.Println(canColor(tri, 3)) // true
}
