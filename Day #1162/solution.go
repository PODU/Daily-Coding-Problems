// Root tree, DFS subtree sizes; count non-root nodes with even subtree size (cuttable parent edges). O(n) time, O(n) space.
package main

import "fmt"

var adj [][]int
var answer int

func dfs(u, parent int) int {
	size := 1
	for _, v := range adj[u] {
		if v != parent {
			size += dfs(v, u)
		}
	}
	if parent != -1 && size%2 == 0 {
		answer++
	}
	return size
}

func main() {
	n := 8
	adj = make([][]int, n+1)
	addEdge := func(a, b int) {
		adj[a] = append(adj[a], b)
		adj[b] = append(adj[b], a)
	}
	edges := [][2]int{{1, 2}, {1, 3}, {3, 4}, {3, 5}, {4, 6}, {4, 7}, {4, 8}}
	for _, e := range edges {
		addEdge(e[0], e[1])
	}
	dfs(1, -1)
	fmt.Println(answer)
}
