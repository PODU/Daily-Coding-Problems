// Day 423: Transitive closure via DFS from each vertex. O(V*(V+E)) time, O(V^2) space.
// M[i][j] = 1 iff j is reachable from i (each vertex reaches itself).
package main

import (
	"fmt"
	"strings"
)

var g [][]int
var M [][]int

func dfs(src, u int) {
	M[src][u] = 1
	for _, v := range g[u] {
		if M[src][v] == 0 {
			dfs(src, v)
		}
	}
}

func main() {
	g = [][]int{{0, 1, 3}, {1, 2}, {2}, {3}}
	n := len(g)
	M = make([][]int, n)
	for i := range M {
		M[i] = make([]int, n)
	}
	for i := 0; i < n; i++ {
		dfs(i, i)
	}
	for i := 0; i < n; i++ {
		parts := make([]string, n)
		for j := 0; j < n; j++ {
			parts[j] = fmt.Sprintf("%d", M[i][j])
		}
		fmt.Println("[" + strings.Join(parts, ", ") + "]")
	}
}
