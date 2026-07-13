// Day 1805: Find all bridges via Tarjan's algorithm (disc/low, edge is bridge if low[v] > disc[u]).
// Parallel edges handled by skipping the parent edge once via edge id. O(V+E).
package main

import (
	"fmt"
	"sort"
)

type edge struct{ to, id int }

var (
	adj    [][]edge
	disc   []int
	low    []int
	timer  int
	result [][2]int
)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func dfs(u, parentEdge int) {
	disc[u] = timer
	low[u] = timer
	timer++
	for _, e := range adj[u] {
		if e.id == parentEdge { // skip the single parent edge once
			continue
		}
		if disc[e.to] == -1 {
			dfs(e.to, e.id)
			low[u] = min(low[u], low[e.to])
			if low[e.to] > disc[u] {
				a, b := u, e.to
				if a > b {
					a, b = b, a
				}
				result = append(result, [2]int{a, b})
			}
		} else {
			low[u] = min(low[u], disc[e.to])
		}
	}
}

func main() {
	n := 5
	edges := [][2]int{{0, 1}, {1, 2}, {2, 0}, {1, 3}, {3, 4}}
	adj = make([][]edge, n)
	for i, e := range edges {
		adj[e[0]] = append(adj[e[0]], edge{e[1], i})
		adj[e[1]] = append(adj[e[1]], edge{e[0], i})
	}
	disc = make([]int, n)
	low = make([]int, n)
	for i := range disc {
		disc[i] = -1
		low[i] = -1
	}
	for i := 0; i < n; i++ {
		if disc[i] == -1 {
			dfs(i, -1)
		}
	}
	sort.Slice(result, func(i, j int) bool {
		if result[i][0] != result[j][0] {
			return result[i][0] < result[j][0]
		}
		return result[i][1] < result[j][1]
	})
	for _, b := range result {
		fmt.Printf("%d - %d\n", b[0], b[1])
	}
	// expected: 1 - 3 and 3 - 4
}
