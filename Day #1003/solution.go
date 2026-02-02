// Day 1003: Transitive closure of a graph (adjacency list -> reachability matrix).
// DFS from each vertex marking everything reachable. O(V*(V+E)) time, O(V^2) space.
package main

import "fmt"

var graph [][]int
var M [][]int

func dfs(start, u int) {
	for _, v := range graph[u] {
		if M[start][v] == 0 {
			M[start][v] = 1
			dfs(start, v)
		}
	}
}

func main() {
	graph = [][]int{{0, 1, 3}, {1, 2}, {2}, {3}}
	n := len(graph)
	M = make([][]int, n)
	for i := range M {
		M[i] = make([]int, n)
	}
	for s := 0; s < n; s++ {
		M[s][s] = 1
		dfs(s, s)
	}
	for _, row := range M {
		fmt.Println(row)
	}
}
