// Day 160: Weighted tree diameter. One DFS; each node returns its longest
// downward branch, while we combine the top two branches. Time O(V+E).
package main

import "fmt"

type edge struct {
	to string
	w  int
}

var adj = map[string][]edge{}
var best int64 = 0

func addEdge(u, v string, w int) {
	adj[u] = append(adj[u], edge{v, w})
	adj[v] = append(adj[v], edge{u, w})
}

func dfs(node, parent string) int64 {
	var top1, top2 int64
	for _, e := range adj[node] {
		if e.to == parent {
			continue
		}
		d := int64(e.w) + dfs(e.to, node)
		if d > top1 {
			top2 = top1
			top1 = d
		} else if d > top2 {
			top2 = d
		}
	}
	if top1+top2 > best {
		best = top1 + top2
	}
	return top1
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
	fmt.Println(best) // 17
}
