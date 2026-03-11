// 2-SAT via implication graph + Tarjan SCC. Node 2*v=(v true), 2*v+1=(v false).
// Clause (x OR y): add edges ~x->y and ~y->x. UNSAT iff x and ~x share an SCC.
// Pick literal whose SCC is the sink; verify against clauses. Time O(V+C).
package main

import "fmt"

type clauseT struct {
	v1 int
	n1 bool
	v2 int
	n2 bool
}

var (
	V       int
	adj     [][]int
	comp    []int
	low     []int
	num     []int
	onStk   []bool
	stk     []int
	idx     int
	sccCnt  int
	clauses []clauseT
)

func lit(v int, neg bool) int {
	if neg {
		return 2*v + 1
	}
	return 2 * v
}

func clause(v1 int, n1 bool, v2 int, n2 bool) {
	adj[lit(v1, !n1)] = append(adj[lit(v1, !n1)], lit(v2, n2))
	adj[lit(v2, !n2)] = append(adj[lit(v2, !n2)], lit(v1, n1))
	clauses = append(clauses, clauseT{v1, n1, v2, n2})
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func tarjan(u int) {
	low[u] = idx
	num[u] = idx
	idx++
	stk = append(stk, u)
	onStk[u] = true
	for _, w := range adj[u] {
		if num[w] == -1 {
			tarjan(w)
			low[u] = min(low[u], low[w])
		} else if onStk[w] {
			low[u] = min(low[u], num[w])
		}
	}
	if low[u] == num[u] {
		for {
			x := stk[len(stk)-1]
			stk = stk[:len(stk)-1]
			onStk[x] = false
			comp[x] = sccCnt
			if x == u {
				break
			}
		}
		sccCnt++
	}
}

func satisfies(val []bool) bool {
	for _, c := range clauses {
		a := val[c.v1] != c.n1
		b := val[c.v2] != c.n2
		if !(a || b) {
			return false
		}
	}
	return true
}

func main() {
	V = 3 // a=0, b=1, c=2
	adj = make([][]int, 2*V)
	comp = make([]int, 2*V)
	low = make([]int, 2*V)
	num = make([]int, 2*V)
	onStk = make([]bool, 2*V)
	for i := range comp {
		comp[i] = -1
		num[i] = -1
	}

	// (~c OR b), (b OR c), (~b OR c), (~c OR ~a)
	clause(2, true, 1, false)
	clause(1, false, 2, false)
	clause(1, true, 2, false)
	clause(2, true, 0, true)

	for i := 0; i < 2*V; i++ {
		if num[i] == -1 {
			tarjan(i)
		}
	}

	for v := 0; v < V; v++ {
		if comp[lit(v, false)] == comp[lit(v, true)] {
			fmt.Println("UNSATISFIABLE")
			return
		}
	}

	val := make([]bool, V)
	for v := 0; v < V; v++ {
		val[v] = comp[lit(v, false)] < comp[lit(v, true)]
	}
	if !satisfies(val) {
		for v := 0; v < V; v++ {
			val[v] = comp[lit(v, true)] < comp[lit(v, false)]
		}
	}

	s := func(b bool) string {
		if b {
			return "True"
		}
		return "False"
	}
	fmt.Printf("a = %s, b = %s, c = %s\n", s(val[0]), s(val[1]), s(val[2]))
}
