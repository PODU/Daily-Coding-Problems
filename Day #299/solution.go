// Minimum spanning tree cost via Kruskal + union-find over undirected weighted graph.
// Time: O(E log E), Space: O(V + E).
package main

import (
	"fmt"
	"sort"
)

var parent map[string]string

func find(x string) string {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func unite(a, b string) bool {
	ra, rb := find(a), find(b)
	if ra == rb {
		return false
	}
	parent[ra] = rb
	return true
}

type edge struct {
	w    int
	a, b string
}

func main() {
	pipes := map[string]map[string]int{
		"plant": {"A": 1, "B": 5, "C": 20},
		"A":     {"C": 15},
		"B":     {"C": 10},
		"C":     {},
	}

	parent = map[string]string{}
	for node := range pipes {
		parent[node] = node
	}

	seen := map[string]bool{}
	var edges []edge
	for u, nbrs := range pipes {
		for v, w := range nbrs {
			a, b := u, v
			if a > b {
				a, b = b, a
			}
			key := fmt.Sprintf("%s|%s|%d", a, b, w)
			if !seen[key] {
				seen[key] = true
				edges = append(edges, edge{w, a, b})
			}
		}
	}
	sort.Slice(edges, func(i, j int) bool { return edges[i].w < edges[j].w })

	total := 0
	for _, e := range edges {
		if unite(e.a, e.b) {
			total += e.w
		}
	}
	fmt.Println(total)
}
