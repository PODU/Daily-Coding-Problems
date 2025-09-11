// Transitive closure: DFS from each vertex marking reachable nodes (incl self).
// Time O(V*(V+E)), Space O(V^2) for the matrix.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func dfs(u int, g [][]int, row []int) {
	row[u] = 1
	for _, v := range g[u] {
		if row[v] == 0 {
			dfs(v, g, row)
		}
	}
}

func main() {
	graph := [][]int{{0, 1, 3}, {1, 2}, {2}, {3}}
	n := len(graph)
	M := make([][]int, n)
	for i := range M {
		M[i] = make([]int, n)
		dfs(i, graph, M[i])
	}
	for _, row := range M {
		parts := make([]string, n)
		for j, v := range row {
			parts[j] = strconv.Itoa(v)
		}
		fmt.Println("[" + strings.Join(parts, ", ") + "]")
	}
}
