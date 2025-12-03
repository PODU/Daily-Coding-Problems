// 2-SAT: implication graph + iterative Tarjan SCC. Sat iff no var shares SCC with its negation.
// Time O(n + m), Space O(n + m).
package main

import "fmt"

type twoSat struct {
	n     int
	g     [][]int
	idx   []int
	low   []int
	comp  []int
	onstk []bool
	stk   []int
	cnt   int
	cc    int
}

func newTwoSat(vars int) *twoSat {
	ts := &twoSat{
		n:     vars,
		g:     make([][]int, 2*vars),
		idx:   make([]int, 2*vars),
		low:   make([]int, 2*vars),
		comp:  make([]int, 2*vars),
		onstk: make([]bool, 2*vars),
	}
	for i := range ts.idx {
		ts.idx[i] = -1
		ts.comp[i] = -1
	}
	return ts
}

func node(v int, sign bool) int {
	if sign {
		return 2 * v
	}
	return 2*v + 1
}

func neg(x int) int { return x ^ 1 }

func (ts *twoSat) addClause(v1 int, s1 bool, v2 int, s2 bool) {
	x, y := node(v1, s1), node(v2, s2)
	ts.g[neg(x)] = append(ts.g[neg(x)], y)
	ts.g[neg(y)] = append(ts.g[neg(y)], x)
}

type frame struct{ v, pi int }

func (ts *twoSat) tarjan(start int) {
	work := []frame{{start, 0}}
	for len(work) > 0 {
		top := &work[len(work)-1]
		v := top.v
		if top.pi == 0 {
			ts.idx[v] = ts.cnt
			ts.low[v] = ts.cnt
			ts.cnt++
			ts.stk = append(ts.stk, v)
			ts.onstk[v] = true
		}
		recurse := false
		for i := top.pi; i < len(ts.g[v]); i++ {
			w := ts.g[v][i]
			if ts.idx[w] == -1 {
				top.pi = i + 1
				work = append(work, frame{w, 0})
				recurse = true
				break
			} else if ts.onstk[w] {
				if ts.idx[w] < ts.low[v] {
					ts.low[v] = ts.idx[w]
				}
			}
		}
		if recurse {
			continue
		}
		if ts.low[v] == ts.idx[v] {
			for {
				w := ts.stk[len(ts.stk)-1]
				ts.stk = ts.stk[:len(ts.stk)-1]
				ts.onstk[w] = false
				ts.comp[w] = ts.cc
				if w == v {
					break
				}
			}
			ts.cc++
		}
		work = work[:len(work)-1]
		if len(work) > 0 {
			pv := work[len(work)-1].v
			if ts.low[v] < ts.low[pv] {
				ts.low[pv] = ts.low[v]
			}
		}
	}
}

func (ts *twoSat) solve() ([]bool, bool) {
	for i := 0; i < 2*ts.n; i++ {
		if ts.idx[i] == -1 {
			ts.tarjan(i)
		}
	}
	assign := make([]bool, ts.n)
	for v := 0; v < ts.n; v++ {
		if ts.comp[2*v] == ts.comp[2*v+1] {
			return nil, false
		}
		assign[v] = ts.comp[2*v] < ts.comp[2*v+1]
	}
	return assign, true
}

func main() {
	// Variables a=0, b=1, c=2. sign true=positive, false=negated.
	// (!c OR b) AND (b OR c) AND (!b OR c) AND (!c OR !a)
	ts := newTwoSat(3)
	ts.addClause(2, false, 1, true)
	ts.addClause(1, true, 2, true)
	ts.addClause(1, false, 2, true)
	ts.addClause(2, false, 0, false)

	assign, ok := ts.solve()
	if !ok {
		fmt.Println("UNSATISFIABLE")
		return
	}
	names := []string{"a", "b", "c"}
	out := ""
	for i := 0; i < 3; i++ {
		if i > 0 {
			out += ", "
		}
		val := "False"
		if assign[i] {
			val = "True"
		}
		out += fmt.Sprintf("%s = %s", names[i], val)
	}
	fmt.Println(out)
}
