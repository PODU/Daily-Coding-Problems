// Day 1286: Find all bridges in an undirected graph (Tarjan's low-link DFS).
// Time O(V + E), Space O(V + E).
package main

import (
	"fmt"
	"sort"
)

var adj [][]int
var disc, low []int
var timer int
var bridges [][2]int

func minI(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func dfs(u, parent int) {
	timer++
	disc[u], low[u] = timer, timer
	skipped := false
	for _, v := range adj[u] {
		if v == parent && !skipped {
			skipped = true
			continue
		}
		if disc[v] == 0 {
			dfs(v, u)
			low[u] = minI(low[u], low[v])
			if low[v] > disc[u] {
				a, b := u, v
				if a > b {
					a, b = b, a
				}
				bridges = append(bridges, [2]int{a, b})
			}
		} else {
			low[u] = minI(low[u], disc[v])
		}
	}
}

func main() {
	n := 5
	edges := [][2]int{{0, 1}, {1, 2}, {2, 0}, {1, 3}, {3, 4}}
	adj = make([][]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	disc = make([]int, n)
	low = make([]int, n)
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
	for _, b := range bridges {
		fmt.Printf("%d - %d\n", b[0], b[1])
	}
}
