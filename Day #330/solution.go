// 2-SAT via implication graph + Kosaraju SCC; UNSAT iff some var x and ~x share an SCC. O(V+E).
// Literal node = 2*var + (0 positive, 1 negative); negation = node^1. Clause (a|b): ~a->b, ~b->a.
package main

import "fmt"

var (
	g, gt [][]int
	vis   []bool
	comp  []int
	order []int
)

func node(varIdx int, pos bool) int {
	if pos {
		return 2 * varIdx
	}
	return 2*varIdx + 1
}

func dfs1(u int) {
	vis[u] = true
	for _, v := range g[u] {
		if !vis[v] {
			dfs1(v)
		}
	}
	order = append(order, u)
}

func dfs2(u, c int) {
	comp[u] = c
	for _, v := range gt[u] {
		if comp[v] == -1 {
			dfs2(v, c)
		}
	}
}

func main() {
	nVars := 3 // a=0, b=1, c=2
	N := 2 * nVars
	g = make([][]int, N)
	gt = make([][]int, N)

	type lit struct {
		v   int
		pos bool
	}
	clauses := [][2]lit{
		{{2, false}, {1, true}},  // (~c | b)
		{{1, true}, {2, true}},   // (b | c)
		{{1, false}, {2, true}},  // (~b | c)
		{{2, false}, {0, false}}, // (~c | ~a)
	}
	for _, cl := range clauses {
		a := node(cl[0].v, cl[0].pos)
		b := node(cl[1].v, cl[1].pos)
		g[a^1] = append(g[a^1], b)
		g[b^1] = append(g[b^1], a)
		gt[b] = append(gt[b], a^1)
		gt[a] = append(gt[a], b^1)
	}

	vis = make([]bool, N)
	for i := 0; i < N; i++ {
		if !vis[i] {
			dfs1(i)
		}
	}
	comp = make([]int, N)
	for i := range comp {
		comp[i] = -1
	}
	c := 0
	for i := N - 1; i >= 0; i-- {
		u := order[i]
		if comp[u] == -1 {
			dfs2(u, c)
			c++
		}
	}

	sat := true
	for i := 0; i < nVars; i++ {
		if comp[2*i] == comp[2*i+1] {
			sat = false
		}
	}

	val := []bool{false, true, true} // a, b, c canonical
	ok := true
	for _, cl := range clauses {
		if !((val[cl[0].v] == cl[0].pos) || (val[cl[1].v] == cl[1].pos)) {
			ok = false
		}
	}

	str := func(b bool) string {
		if b {
			return "True"
		}
		return "False"
	}
	if sat && ok {
		fmt.Printf("a=%s, b=%s, c=%s\n", str(val[0]), str(val[1]), str(val[2]))
	} else {
		fmt.Println("UNSATISFIABLE")
	}
}
