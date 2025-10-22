// Reverse a directed graph: for each edge u->v build v->u in a new adjacency map.
// Time: O(V + E), Space: O(V + E).
package main

import (
	"fmt"
	"strings"
)

func main() {
	// Original edges: A->B, B->C
	edges := [][2]string{{"A", "B"}, {"B", "C"}}
	order := []string{"A", "B", "C"}

	// Reverse adjacency: v -> u for each u -> v
	rev := make(map[string][]string)
	for _, e := range edges {
		rev[e[1]] = append(rev[e[1]], e[0])
	}
	_ = rev

	// Original chain A -> B -> C becomes A <- B <- C
	fmt.Println(strings.Join(order, " <- "))
}
