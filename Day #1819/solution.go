// Longest weighted path (diameter) in a tree via two DFS passes.
// DFS from any node finds one endpoint; DFS from it finds the other + path. Time: O(V+E). Space: O(V).
package main

import (
	"fmt"
	"strings"
)

type edge struct {
	to string
	w  int
}

var g = map[string][]edge{}

func add(a, b string, w int) {
	g[a] = append(g[a], edge{b, w})
	g[b] = append(g[b], edge{a, w})
}

func farthest(src string) (string, int, map[string]string) {
	parent := map[string]string{}
	visited := map[string]bool{}
	bestNode, bestDist := src, 0
	var dfs func(u string, d int)
	dfs = func(u string, d int) {
		visited[u] = true
		if d > bestDist {
			bestDist, bestNode = d, u
		}
		for _, e := range g[u] {
			if !visited[e.to] {
				parent[e.to] = u
				dfs(e.to, d+e.w)
			}
		}
	}
	dfs(src, 0)
	return bestNode, bestDist, parent
}

func main() {
	add("a", "b", 3)
	add("a", "c", 5)
	add("a", "d", 8)
	add("d", "e", 2)
	add("d", "f", 4)
	add("e", "g", 1)
	add("e", "h", 1)

	u, _, _ := farthest("a")
	v, length, parent := farthest(u)

	var path []string
	for cur := v; ; cur = parent[cur] {
		path = append(path, cur)
		if cur == u {
			break
		}
	}
	fmt.Printf("%s, with a length of %d\n", strings.Join(path, " -> "), length)
	// c -> a -> d -> f, with a length of 17
}
