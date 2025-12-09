// Day 721: Maximum-weight spanning tree.
// Approach: Kruskal with edges sorted by DECREASING weight + union-find.
// Time: O(E log E), Space: O(V + E).
package main

import (
	"fmt"
	"sort"
)

type dsu struct{ parent, rank []int }

func newDSU(n int) *dsu {
	p := make([]int, n)
	for i := range p {
		p[i] = i
	}
	return &dsu{p, make([]int, n)}
}
func (d *dsu) find(x int) int {
	for d.parent[x] != x {
		d.parent[x] = d.parent[d.parent[x]]
		x = d.parent[x]
	}
	return x
}
func (d *dsu) unite(a, b int) bool {
	a, b = d.find(a), d.find(b)
	if a == b {
		return false
	}
	if d.rank[a] < d.rank[b] {
		a, b = b, a
	}
	d.parent[b] = a
	if d.rank[a] == d.rank[b] {
		d.rank[a]++
	}
	return true
}

func maxSpanningTree(n int, edges [][3]int) int {
	sort.Slice(edges, func(i, j int) bool { return edges[i][2] > edges[j][2] })
	d := newDSU(n)
	total, used := 0, 0
	for _, e := range edges {
		if d.unite(e[0], e[1]) {
			total += e[2]
			used++
		}
	}
	if used == n-1 {
		return total
	}
	return -1
}

func main() {
	n := 4
	edges := [][3]int{{0, 1, 1}, {0, 2, 2}, {0, 3, 3}, {1, 2, 4}, {2, 3, 5}}
	fmt.Println("Maximum spanning tree weight:", maxSpanningTree(n, edges))
}
