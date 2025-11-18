// Tree diameter with edge weights: DFS returning max downward path; global best
// = sum of two largest child paths. Time O(N), Space O(N).
package main

import "fmt"

type edge struct {
	to int
	w  int64
}

var adj [][]edge
var best int64

func dfs(u, parent int) int64 {
	var max1, max2 int64
	for _, e := range adj[u] {
		if e.to == parent {
			continue
		}
		path := dfs(e.to, u) + e.w
		if path > max1 {
			max2, max1 = max1, path
		} else if path > max2 {
			max2 = path
		}
	}
	if max1+max2 > best {
		best = max1 + max2
	}
	return max1
}

func main() {
	// a0 b1 c2 d3 e4 f5 g6 h7
	adj = make([][]edge, 8)
	add := func(u, v int, w int64) {
		adj[u] = append(adj[u], edge{v, w})
		adj[v] = append(adj[v], edge{u, w})
	}
	add(0, 1, 3)
	add(0, 2, 5)
	add(0, 3, 8)
	add(3, 4, 2)
	add(3, 5, 4)
	add(4, 6, 1)
	add(4, 7, 1)
	dfs(0, -1)
	fmt.Println(best)
}
