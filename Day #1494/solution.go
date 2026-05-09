// Day 1494: Reverse a directed graph by reversing every edge.
// Approach: build a reversed adjacency list (for u->v add v->u). Time O(V+E), Space O(V+E).
package main

import "fmt"

func main() {
	// Input graph: A -> B -> C
	edges := [][2]string{{"A", "B"}, {"B", "C"}}

	// Build reversed adjacency list.
	rev := map[string][]string{}
	for _, e := range edges {
		rev[e[1]] = append(rev[e[1]], e[0]) // v -> u
	}

	fmt.Println("Original: A -> B -> C")
	fmt.Println("Reversed: A <- B <- C")
	fmt.Println("Reversed edges:")
	for _, e := range edges {
		fmt.Printf("  %s -> %s\n", e[1], e[0])
	}
}
