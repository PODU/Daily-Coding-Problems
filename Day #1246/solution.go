// Day 1246: Maximum weight spanning tree.
// Approach: Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// Time: O(E log E), Space: O(V + E).
package main

import (
	"fmt"
	"sort"
)

type dsu struct {
	p, r []int
}

func newDSU(n int) *dsu {
	d := &dsu{p: make([]int, n), r: make([]int, n)}
	for i := range d.p {
		d.p[i] = i
	}
	return d
}

func (d *dsu) find(x int) int {
	for d.p[x] != x {
		d.p[x] = d.p[d.p[x]]
		x = d.p[x]
	}
	return x
}

func (d *dsu) unite(a, b int) bool {
	a, b = d.find(a), d.find(b)
	if a == b {
		return false
	}
	if d.r[a] < d.r[b] {
		a, b = b, a
	}
	d.p[b] = a
	if d.r[a] == d.r[b] {
		d.r[a]++
	}
	return true
}

// edges: [weight, u, v]
func maxSpanningTree(n int, edges [][3]int) int {
	sort.Slice(edges, func(i, j int) bool { return edges[i][0] > edges[j][0] })
	d := newDSU(n)
	total := 0
	for _, e := range edges {
		if d.unite(e[1], e[2]) {
			total += e[0]
		}
	}
	return total
}

func main() {
	edges := [][3]int{{1, 0, 1}, {2, 1, 2}, {3, 2, 3}, {4, 0, 3}, {5, 0, 2}}
	fmt.Println(maxSpanningTree(4, edges)) // 11
}
