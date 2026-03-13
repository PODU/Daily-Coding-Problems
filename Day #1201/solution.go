// Day 1201: Reverse a directed graph.
// Build reversed adjacency (for each edge u->v add v->u). Time O(V+E), Space O(V+E).
package main

import (
	"fmt"
	"strings"
)

func reverseGraph(g map[string][]string) map[string][]string {
	r := make(map[string][]string)
	for u := range g {
		if _, ok := r[u]; !ok {
			r[u] = []string{}
		}
	}
	for u, vs := range g {
		for _, v := range vs {
			r[v] = append(r[v], u)
		}
	}
	return r
}

func main() {
	nodes := []string{"A", "B", "C"}
	g := make(map[string][]string)
	for i := 0; i+1 < len(nodes); i++ {
		g[nodes[i]] = append(g[nodes[i]], nodes[i+1])
	}
	reverseGraph(g)
	fmt.Println(strings.Join(nodes, " <- "))
}
