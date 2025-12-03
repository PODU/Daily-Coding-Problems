// Remove max edges so each resulting subtree has an even node count.
// DFS subtree sizes; count non-root nodes whose subtree size is even (each = one removable edge above it). O(n) time/space.
package main

import "fmt"

var adj [][]int
var removable int

func dfs(u, parent int) int {
	size := 1
	for _, v := range adj[u] {
		if v != parent {
			size += dfs(v, u)
		}
	}
	if parent != -1 && size%2 == 0 {
		removable++
	}
	return size
}

func main() {
	n := 8
	adj = make([][]int, n+1)
	edges := [][2]int{{1, 2}, {1, 3}, {3, 4}, {3, 5}, {4, 6}, {4, 7}, {4, 8}}
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	dfs(1, -1)
	fmt.Println(removable)
}
