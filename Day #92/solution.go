// Day 92: Topological sort (Kahn's algorithm) over a prerequisite graph.
// Returns a valid course order or nil if a cycle exists. O(V+E).
package main

import (
	"fmt"
	"sort"
)

func courseOrder(prereqs map[string][]string) []string {
	indeg := map[string]int{}
	adj := map[string][]string{}
	for c := range prereqs {
		indeg[c] = indeg[c]
	}
	for c, pres := range prereqs {
		for _, p := range pres {
			adj[p] = append(adj[p], c)
			indeg[c]++
		}
	}
	var ready []string
	for c, d := range indeg {
		if d == 0 {
			ready = append(ready, c)
		}
	}
	sort.Strings(ready)
	var order []string
	for len(ready) > 0 {
		c := ready[0]
		ready = ready[1:]
		order = append(order, c)
		for _, n := range adj[c] {
			indeg[n]--
			if indeg[n] == 0 {
				ready = append(ready, n)
				sort.Strings(ready)
			}
		}
	}
	if len(order) != len(prereqs) {
		return nil
	}
	return order
}

func main() {
	g := map[string][]string{
		"CSC300": {"CSC100", "CSC200"}, "CSC200": {"CSC100"}, "CSC100": {}}
	fmt.Println(courseOrder(g)) // [CSC100 CSC200 CSC300]
}
