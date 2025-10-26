// Day 492: Graph m-colorability via backtracking.
// Assign each vertex a color 1..k, ensuring no adjacent pair (adjacency matrix) matches.
// Time: O(k^V) worst case, Space: O(V) for the color assignment + recursion stack.
package main

import "fmt"

func isSafe(v int, graph [][]int, colors []int, c int) bool {
	for u := 0; u < len(graph); u++ {
		if graph[v][u] == 1 && colors[u] == c {
			return false
		}
	}
	return true
}

func solve(v int, graph [][]int, k int, colors []int) bool {
	if v == len(graph) {
		return true
	}
	for c := 1; c <= k; c++ {
		if isSafe(v, graph, colors, c) {
			colors[v] = c
			if solve(v+1, graph, k, colors) {
				return true
			}
			colors[v] = 0
		}
	}
	return false
}

func canColor(graph [][]int, k int) bool {
	return solve(0, graph, k, make([]int, len(graph)))
}

func main() {
	// Triangle K3: every pair adjacent.
	graph := [][]int{
		{0, 1, 1},
		{1, 0, 1},
		{1, 1, 0},
	}
	fmt.Printf("k=2 colorable: %t\n", canColor(graph, 2))
	fmt.Printf("k=3 colorable: %t\n", canColor(graph, 3))
}
