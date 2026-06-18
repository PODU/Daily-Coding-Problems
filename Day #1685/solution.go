// Day 1685: Merge ranked preference lists -> topological sort (Kahn's BFS, FIFO,
// first-seen tie-break). Each adjacent pair in a list is an edge. Time O(V+E).
package main

import "fmt"

func interleave(lists [][]int) []int {
	var order []int
	seen := map[int]bool{}
	adj := map[int][]int{}
	edges := map[[2]int]bool{}
	indeg := map[int]int{}

	for _, lst := range lists {
		for _, x := range lst {
			if !seen[x] {
				seen[x] = true
				order = append(order, x)
				indeg[x] = indeg[x]
			}
		}
		for i := 0; i+1 < len(lst); i++ {
			a, b := lst[i], lst[i+1]
			if !edges[[2]int{a, b}] {
				edges[[2]int{a, b}] = true
				adj[a] = append(adj[a], b)
				indeg[b]++
			}
		}
	}
	var q []int
	for _, x := range order {
		if indeg[x] == 0 {
			q = append(q, x)
		}
	}
	var res []int
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
	fmt.Println(interleave(lists)) // [2 1 6 7 3 9 5]
}
