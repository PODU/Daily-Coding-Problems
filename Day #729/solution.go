// Day 729: Interleave ranked preference lists into one consistent playlist.
// Approach: Build precedence DAG (consecutive pairs), Kahn topological sort (FIFO,
// first-appearance tie-break). Time: O(V + E), Space: O(V + E).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func interleave(lists [][]int) []int {
	adj := make(map[int][]int)
	indeg := make(map[int]int)
	edges := make(map[[2]int]bool)
	order := []int{}
	for _, lst := range lists {
		for _, x := range lst {
			if _, ok := indeg[x]; !ok {
				indeg[x] = 0
				adj[x] = []int{}
				order = append(order, x)
			}
		}
		for i := 0; i+1 < len(lst); i++ {
			e := [2]int{lst[i], lst[i+1]}
			if !edges[e] {
				edges[e] = true
				adj[lst[i]] = append(adj[lst[i]], lst[i+1])
				indeg[lst[i+1]]++
			}
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
	res := interleave([][]int{{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}})
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
