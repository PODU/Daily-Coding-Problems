// Day 1065: Reverse a directed graph (reverse every edge).
// Approach: iterate all edges u->v, add v->u to reversed adjacency. Time O(V+E), Space O(V+E).
package main

import (
	"fmt"
	"sort"
)

func reverseGraph(g map[string][]string) map[string][]string {
	r := make(map[string][]string)
	for u := range g {
		if _, ok := r[u]; !ok {
			r[u] = []string{} // keep isolated nodes
		}
	}
	for u, neighbors := range g {
		for _, v := range neighbors {
			r[v] = append(r[v], u)
		}
	}
	return r
}

func main() {
	// A -> B -> C
	g := map[string][]string{"A": {"B"}, "B": {"C"}, "C": {}}
	r := reverseGraph(g)
	// Reversed: B -> A, C -> B  ("A <- B <- C")
	fmt.Println("A <- B <- C")
	keys := make([]string, 0, len(r))
	for k := range r {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, u := range keys {
		for _, v := range r[u] {
			fmt.Printf("%s -> %s\n", u, v)
		}
	}
}
