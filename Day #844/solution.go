// Day 844: 2-SAT via implication graph + Kosaraju SCC.
// Clause (x OR y) adds !x->y and !y->x. SAT iff no var shares an SCC with its negation. O(V+E).
package main

import "fmt"

type twoSat struct {
	n   int
	adj [][]int
}

func newTwoSat(n int) *twoSat { return &twoSat{n: n, adj: make([][]int, 2*n)} }

func node(v int, neg bool) int {
	if neg {
		return 2*v + 1
	}
	return 2 * v
}

func (t *twoSat) addClause(va int, na bool, vb int, nb bool) {
	a, b := node(va, na), node(vb, nb)
	t.adj[a^1] = append(t.adj[a^1], b)
	t.adj[b^1] = append(t.adj[b^1], a)
}

func (t *twoSat) solve() ([]bool, bool) {
	n2 := 2 * t.n
	vis := make([]bool, n2)
	order := []int{}
	var dfs1 func(u int)
	dfs1 = func(u int) {
		vis[u] = true
		for _, v := range t.adj[u] {
			if !vis[v] {
				dfs1(v)
			}
		}
		order = append(order, u)
	}
	for i := 0; i < n2; i++ {
		if !vis[i] {
			dfs1(i)
		}
	}
	radj := make([][]int, n2)
	for u := 0; u < n2; u++ {
		for _, v := range t.adj[u] {
			radj[v] = append(radj[v], u)
		}
	}
	comp := make([]int, n2)
	for i := range comp {
		comp[i] = -1
	}
	c := 0
	var dfs2 func(u, cc int)
	dfs2 = func(u, cc int) {
		comp[u] = cc
		for _, v := range radj[u] {
			if comp[v] == -1 {
				dfs2(v, cc)
			}
		}
	}
	for i := n2 - 1; i >= 0; i-- {
		u := order[i]
		if comp[u] == -1 {
			dfs2(u, c)
			c++
		}
	}
	assign := make([]bool, t.n)
	for v := 0; v < t.n; v++ {
		if comp[2*v] == comp[2*v+1] {
			return nil, false
		}
		assign[v] = comp[2*v] > comp[2*v+1]
	}
	return assign, true
}

func tf(b bool) string {
	if b {
		return "True"
	}
	return "False"
}

func main() {
	ts := newTwoSat(3) // a=0,b=1,c=2
	ts.addClause(2, true, 1, false)  // ¬c OR b
	ts.addClause(1, false, 2, false) // b OR c
	ts.addClause(1, true, 2, false)  // ¬b OR c
	ts.addClause(2, true, 0, true)   // ¬c OR ¬a
	a, ok := ts.solve()
	if !ok {
		fmt.Println("False")
		return
	}
	fmt.Printf("a = %s, b = %s, c = %s\n", tf(a[0]), tf(a[1]), tf(a[2]))
}
