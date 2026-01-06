// Day 862: Find all bridges in a connected undirected graph.
// Approach: Tarjan's DFS using discovery times and low-link values.
// Time: O(V + E), Space: O(V + E).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	n := 5
	edges := [][2]int{{0, 1}, {1, 2}, {2, 0}, {1, 3}, {3, 4}}
	adj := make([][]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	disc := make([]int, n)
	low := make([]int, n)
	timer := 0
	var bridges [][2]int

	var dfs func(u, parent int)
	dfs = func(u, parent int) {
		timer++
		disc[u], low[u] = timer, timer
		for _, v := range adj[u] {
			if v == parent {
				continue
			}
			if disc[v] != 0 {
				if disc[v] < low[u] {
					low[u] = disc[v]
				}
			} else {
				dfs(v, u)
				if low[v] < low[u] {
					low[u] = low[v]
				}
				if low[v] > disc[u] {
					a, b := u, v
					if a > b {
						a, b = b, a
					}
					bridges = append(bridges, [2]int{a, b})
				}
			}
		}
	}
	for i := 0; i < n; i++ {
		if disc[i] == 0 {
			dfs(i, -1)
		}
	}
	sort.Slice(bridges, func(i, j int) bool {
		if bridges[i][0] != bridges[j][0] {
			return bridges[i][0] < bridges[j][0]
		}
		return bridges[i][1] < bridges[j][1]
	})
	var parts []string
	for _, b := range bridges {
		parts = append(parts, fmt.Sprintf("(%d, %d)", b[0], b[1]))
	}
	fmt.Println("Bridges: " + strings.Join(parts, " ")) // (1, 3) (3, 4)
}
