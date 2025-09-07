// Maximum spanning tree: Kruskal with edges sorted by descending weight + union-find.
// Time: O(E log E), Space: O(V).
package main

import (
	"fmt"
	"sort"
	"strings"
)

var parent []int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func unite(a, b int) bool {
	ra, rb := find(a), find(b)
	if ra == rb {
		return false
	}
	parent[ra] = rb
	return true
}

func maxSpanningTree(n int, edges [][3]int) (int, [][2]int) {
	sort.Slice(edges, func(i, j int) bool { return edges[i][2] > edges[j][2] })
	parent = make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	total := 0
	var chosen [][2]int
	for _, e := range edges {
		if unite(e[0], e[1]) {
			total += e[2]
			chosen = append(chosen, [2]int{e[0], e[1]})
		}
	}
	return total, chosen
}

func main() {
	n := 4
	edges := [][3]int{{0, 1, 1}, {1, 2, 2}, {2, 3, 3}, {0, 3, 4}, {0, 2, 5}}
	total, chosen := maxSpanningTree(n, edges)
	fmt.Println("Max spanning tree weight:", total) // 11
	parts := []string{}
	for _, e := range chosen {
		parts = append(parts, fmt.Sprintf("(%d-%d)", e[0], e[1]))
	}
	fmt.Println("Edges:", strings.Join(parts, " "))
}
