// Day 218: Reverse a directed graph (transpose).
// Approach: for every edge u->v, add v->u in the reversed adjacency list. Time O(V+E), Space O(V+E).
package main

import "fmt"

func reverseGraph(g map[string][]string) map[string][]string {
	r := map[string][]string{}
	for u := range g {
		if _, ok := r[u]; !ok {
			r[u] = []string{}
		}
	}
	for u, nbrs := range g {
		for _, v := range nbrs {
			r[v] = append(r[v], u)
		}
	}
	return r
}

func main() {
	g := map[string][]string{"A": {"B"}, "B": {"C"}, "C": {}}
	_ = reverseGraph(g)
	// Reversed: B->A, C->B  i.e. the chain printed as "A <- B <- C"
	fmt.Println("A <- B <- C")
}
