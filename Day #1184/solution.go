// Day 1184: Interleave ranked lists into one playlist respecting every ordering.
// Build a DAG of consecutive-preference edges and run Kahn topological sort (FIFO).
// Time O(V + E), Space O(V + E).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func interleave(lists [][]int) []int {
	var order []int
	known := map[int]bool{}
	adj := map[int][]int{}
	indeg := map[int]int{}
	edges := map[[2]int]bool{}

	for _, l := range lists {
		for _, v := range l {
			if !known[v] {
				known[v] = true
				order = append(order, v)
				if _, ok := adj[v]; !ok {
					adj[v] = nil
				}
				indeg[v] += 0
			}
		}
		for i := 0; i+1 < len(l); i++ {
			u, w := l[i], l[i+1]
			e := [2]int{u, w}
			if u != w && !edges[e] {
				edges[e] = true
				adj[u] = append(adj[u], w)
				indeg[w]++
			}
		}
	}

	var q []int
	for _, v := range order {
		if indeg[v] == 0 {
			q = append(q, v)
		}
	}
	var res []int
	for len(q) > 0 {
		v := q[0]
		q = q[1:]
		res = append(res, v)
		for _, w := range adj[v] {
			indeg[w]--
			if indeg[w] == 0 {
				q = append(q, w)
			}
		}
	}
	return res
}

func main() {
	lists := [][]int{{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}}
	r := interleave(lists)
	strs := make([]string, len(r))
	for i, x := range r {
		strs[i] = strconv.Itoa(x)
	}
	fmt.Printf("[%s]\n", strings.Join(strs, ", ")) // [2, 1, 6, 7, 3, 9, 5]
}
