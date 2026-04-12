// Weighted tree diameter: DFS, at each node track two largest downward child path sums;
// diameter = max over nodes of (sum of two largest). Time O(V+E), Space O(V+E).
package main

import "fmt"

type edge struct{ to, w int }

var adj [][]edge
var best int

func dfs(u, parent int) int {
	max1, max2 := 0, 0 // two largest downward path sums
	for _, e := range adj[u] {
		if e.to == parent {
			continue
		}
		down := dfs(e.to, u) + e.w
		if down > max1 {
			max2, max1 = max1, down
		} else if down > max2 {
			max2 = down
		}
	}
	if max1+max2 > best {
		best = max1 + max2
	}
	return max1
}

func add(a, b, w int) {
	adj[a] = append(adj[a], edge{b, w})
	adj[b] = append(adj[b], edge{a, w})
}

func main() {
	n := 8 // a..h -> 0..7
	adj = make([][]edge, n)
	add(0, 1, 3) // a-b
	add(0, 2, 5) // a-c
	add(0, 3, 8) // a-d
	add(3, 4, 2) // d-e
	add(3, 5, 4) // d-f
	add(4, 6, 1) // e-g
	add(4, 7, 1) // e-h
	dfs(0, -1)
	fmt.Println(best)
}
