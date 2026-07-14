// Split students into two teams so no enemies share a team = graph 2-coloring (bipartite check).
// BFS coloring over each component. Time: O(V+E). Space: O(V).
package main

import (
	"fmt"
	"sort"
)

func divideTeams(g map[int][]int) ([]int, []int, bool) {
	color := map[int]int{}
	keys := []int{}
	for k := range g {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	for _, s := range keys {
		if _, ok := color[s]; ok {
			continue
		}
		color[s] = 0
		q := []int{s}
		for len(q) > 0 {
			u := q[0]
			q = q[1:]
			for _, v := range g[u] {
				if _, ok := color[v]; !ok {
					color[v] = color[u] ^ 1
					q = append(q, v)
				} else if color[v] == color[u] {
					return nil, nil, false
				}
			}
		}
	}
	var a, b []int
	for n, c := range color {
		if c == 0 {
			a = append(a, n)
		} else {
			b = append(b, n)
		}
	}
	sort.Ints(a)
	sort.Ints(b)
	return a, b, true
}

func main() {
	students := map[int][]int{0: {3}, 1: {2}, 2: {1, 4}, 3: {0, 4, 5}, 4: {2, 3}, 5: {3}}
	a, b, ok := divideTeams(students)
	if ok {
		fmt.Printf("{%v} and {%v}\n", join(a), join(b))
	} else {
		fmt.Println("False")
	}
	// {0, 1, 4, 5} and {2, 3}
}

func join(s []int) string {
	out := ""
	for i, x := range s {
		if i > 0 {
			out += ", "
		}
		out += fmt.Sprintf("%d", x)
	}
	return out
}
