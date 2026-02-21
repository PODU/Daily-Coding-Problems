// Day 1108: Detect a cycle in an undirected graph using Union-Find.
// If an edge connects two already-connected vertices, there's a cycle.
// Time: O(E * alpha(V)). Space: O(V).
package main

import "fmt"

type DSU struct {
	parent, rank []int
}

func NewDSU(n int) *DSU {
	p := make([]int, n)
	for i := range p {
		p[i] = i
	}
	return &DSU{p, make([]int, n)}
}

func (d *DSU) find(x int) int {
	for d.parent[x] != x {
		d.parent[x] = d.parent[d.parent[x]]
		x = d.parent[x]
	}
	return x
}

func (d *DSU) unite(a, b int) bool {
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

func hasCycle(n int, edges [][2]int) bool {
	d := NewDSU(n)
	for _, e := range edges {
		if !d.unite(e[0], e[1]) {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(hasCycle(3, [][2]int{{0, 1}, {1, 2}, {2, 0}})) // true
	fmt.Println(hasCycle(4, [][2]int{{0, 1}, {1, 2}, {2, 3}})) // false
}
