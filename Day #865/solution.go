// Day 865: Minimum cost to connect all houses to the plant = MST cost.
// Approach: Kruskal's algorithm with union-find over all pipe edges.
// Time: O(E log E), Space: O(V + E).
package main

import (
	"fmt"
	"sort"
)

type edge struct {
	cost int
	u, v string
}

var parent []int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func main() {
	pipes := map[string]map[string]int{
		"plant": {"A": 1, "B": 5, "C": 20},
		"A":     {"C": 15},
		"B":     {"C": 10},
		"C":     {},
	}
	id := map[string]int{}
	getID := func(s string) int {
		if _, ok := id[s]; !ok {
			id[s] = len(id)
		}
		return id[s]
	}
	var edges []edge
	for a, nbrs := range pipes {
		getID(a)
		for b, c := range nbrs {
			getID(b)
			edges = append(edges, edge{c, a, b})
		}
	}
	sort.Slice(edges, func(i, j int) bool { return edges[i].cost < edges[j].cost })

	parent = make([]int, len(id))
	for i := range parent {
		parent[i] = i
	}
	total := 0
	for _, e := range edges {
		ru, rv := find(id[e.u]), find(id[e.v])
		if ru != rv {
			parent[ru] = rv
			total += e.cost
		}
	}
	fmt.Println(total) // 16
}
