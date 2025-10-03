// Merge ranked lists via topological sort: edge a->b for consecutive a,b in any list; Kahn's with FIFO queue. O(V+E).
package main

import (
	"fmt"
	"strings"
)

func merge(lists [][]int) []int {
	order := []int{} // first-appearance order
	seen := map[int]bool{}
	adj := map[int][]int{}
	indeg := map[int]int{}
	for _, l := range lists {
		for _, x := range l {
			if !seen[x] {
				seen[x] = true
				order = append(order, x)
				if _, ok := indeg[x]; !ok {
					indeg[x] = 0
				}
			}
		}
		for i := 0; i+1 < len(l); i++ {
			adj[l[i]] = append(adj[l[i]], l[i+1])
			indeg[l[i+1]]++
		}
	}
	q := []int{}
	for _, x := range order {
		if indeg[x] == 0 {
			q = append(q, x)
		}
	}
	res := []int{}
	for len(q) > 0 {
		u := q[0]
		q = q[1:]
		res = append(res, u)
		for _, v := range adj[u] {
			indeg[v]--
			if indeg[v] == 0 {
				q = append(q, v)
			}
		}
	}
	return res
}

func main() {
	lists := [][]int{{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}}
	r := merge(lists)
	parts := make([]string, len(r))
	for i, x := range r {
		parts[i] = fmt.Sprintf("%d", x)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
