// Day 945: Longest path (diameter) in a weighted tree.
// DFS: at each node combine its two deepest downward branches. Time O(V+E), Space O(V).
package main

import "fmt"

type edge struct {
	to string
	w  int
}

var adj = map[string][]edge{}
var best = 0

func addEdge(u, v string, w int) {
	adj[u] = append(adj[u], edge{v, w})
	adj[v] = append(adj[v], edge{u, w})
}

// Returns the longest downward path length from node (excluding edge above it).
func dfs(node, parent string) int {
	max1, max2 := 0, 0
	for _, e := range adj[node] {
		if e.to == parent {
			continue
		}
		d := dfs(e.to, node) + e.w
		if d > max1 {
			max2 = max1
			max1 = d
		} else if d > max2 {
			max2 = d
		}
	}
	if max1+max2 > best {
		best = max1 + max2
	}
	return max1
}

func main() {
	addEdge("a", "b", 3)
	addEdge("a", "c", 5)
	addEdge("a", "d", 8)
	addEdge("d", "e", 2)
	addEdge("d", "f", 4)
	addEdge("e", "g", 1)
	addEdge("e", "h", 1)
	dfs("a", "")
	fmt.Println(best) // 17 (path c -> a -> d -> f)
}
