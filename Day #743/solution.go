// Find all bridges with Tarjan's DFS: edge (u,v) is a bridge if low[v] > disc[u].
// Time: O(V + E), Space: O(V + E).
package main

import (
	"fmt"
	"sort"
)

var (
	adj     [][]int
	disc    []int
	low     []int
	bridges [][2]int
	timer   int
)

func dfs(u, parent int) {
	timer++
	disc[u], low[u] = timer, timer
	skippedParent := false
	for _, v := range adj[u] {
		if v == parent && !skippedParent {
			skippedParent = true
			continue
		}
		if disc[v] == 0 {
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
		} else if disc[v] < low[u] {
			low[u] = disc[v]
		}
	}
}

func main() {
	n := 5
	adj = make([][]int, n)
	edges := [][2]int{{0, 1}, {1, 2}, {2, 0}, {1, 3}, {3, 4}}
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
		fmt.Printf("(%d, %d)\n", b[0], b[1])
	}
	// (1, 3)
	// (3, 4)
}
