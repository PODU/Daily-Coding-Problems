// Day 998: Graph k-colorability (adjacency matrix).
// Backtracking: try each color per vertex, skipping colors used by neighbors.
// O(k^V) worst case, O(V) extra space.
package main

import "fmt"

func canColor(graph [][]int, k int) bool {
	n := len(graph)
	colors := make([]int, n)
	var ok func(v, c int) bool
	ok = func(v, c int) bool {
		for u := 0; u < n; u++ {
			if graph[v][u] == 1 && colors[u] == c {
				return false
			}
		}
		return true
	}
	var solve func(v int) bool
	solve = func(v int) bool {
		if v == n {
			return true
		}
		for c := 1; c <= k; c++ {
			if ok(v, c) {
				colors[v] = c
				if solve(v + 1) {
					return true
				}
				colors[v] = 0
			}
		}
		return false
	}
	return solve(0)
}

func main() {
	triangle := [][]int{{0, 1, 1}, {1, 0, 1}, {1, 1, 0}}
	fmt.Println(canColor(triangle, 2)) // false
	fmt.Println(canColor(triangle, 3)) // true
}
