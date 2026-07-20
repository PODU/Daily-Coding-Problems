// Day 1847: 2-SAT solver. Build implication graph, find SCCs (Kosaraju), check x and ¬x differ.
// Time O(V + C), Space O(V + C). Literal node = 2*var + (negated?1:0); neg(x)=x^1.
package main

import (
	"fmt"
	"strings"
)

type TwoSat struct {
	n          int
	adj, adjT  [][]int
	used       []bool
	comp       []int
	order      []int
}

func NewTwoSat(n int) *TwoSat {
	return &TwoSat{n: n, adj: make([][]int, 2*n), adjT: make([][]int, 2*n)}
}

func (t *TwoSat) addOr(u, v int) {
	t.adj[u^1] = append(t.adj[u^1], v)
	t.adjT[v] = append(t.adjT[v], u^1)
	t.adj[v^1] = append(t.adj[v^1], u)
	t.adjT[u] = append(t.adjT[u], v^1)
}

func (t *TwoSat) dfs1(v int) {
	t.used[v] = true
	for _, to := range t.adj[v] {
		if !t.used[to] {
			t.dfs1(to)
		}
	}
	t.order = append(t.order, v)
}

func (t *TwoSat) dfs2(v, c int) {
	t.comp[v] = c
	for _, to := range t.adjT[v] {
		if t.comp[to] == -1 {
			t.dfs2(to, c)
		}
	}
}

func (t *TwoSat) solve() ([]bool, bool) {
	N := 2 * t.n
	t.used = make([]bool, N)
	for i := 0; i < N; i++ {
		if !t.used[i] {
			t.dfs1(i)
		}
	}
	t.comp = make([]int, N)
	for i := range t.comp {
		t.comp[i] = -1
	}
	c := 0
	for i := N - 1; i >= 0; i-- {
		v := t.order[i]
		if t.comp[v] == -1 {
			t.dfs2(v, c)
			c++
		}
	}
	res := make([]bool, t.n)
	for i := 0; i < t.n; i++ {
		if t.comp[2*i] == t.comp[2*i+1] {
			return nil, false
		}
		res[i] = t.comp[2*i] > t.comp[2*i+1]
	}
	return res, true
}

func lit(v, neg int) int { return 2*v + neg }

func main() {
	ts := NewTwoSat(3) // a=0, b=1, c=2
	ts.addOr(lit(2, 1), lit(1, 0)) // (¬c OR b)
	ts.addOr(lit(1, 0), lit(2, 0)) // (b OR c)
	ts.addOr(lit(1, 1), lit(2, 0)) // (¬b OR c)
	ts.addOr(lit(2, 1), lit(0, 1)) // (¬c OR ¬a)

	res, ok := ts.solve()
	if !ok {
		fmt.Println("False")
		return
	}
	names := "abc"
	parts := make([]string, 3)
	for i := 0; i < 3; i++ {
		val := "False"
		if res[i] {
			val = "True"
		}
		parts[i] = fmt.Sprintf("%c = %s", names[i], val)
	}
	fmt.Println(strings.Join(parts, ", "))
}
