// Bipartite 2-coloring via BFS over all components (sorted iteration for determinism).
// Time: O(V+E), Space: O(V+E).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func solve(students map[int][]int) ([]int, []int, bool) {
	color := map[int]int{}
	nodes := make([]int, 0, len(students))
	for k := range students {
		nodes = append(nodes, k)
	}
	sort.Ints(nodes)
	for _, start := range nodes {
		if _, ok := color[start]; ok {
			continue
		}
		color[start] = 0
		q := []int{start}
		for len(q) > 0 {
			u := q[0]
			q = q[1:]
			nbrs := append([]int(nil), students[u]...)
			sort.Ints(nbrs)
			for _, v := range nbrs {
				if c, ok := color[v]; !ok {
					color[v] = color[u] ^ 1
					q = append(q, v)
				} else if c == color[u] {
					return nil, nil, false
				}
			}
		}
	}
	var t0, t1 []int
	for k, c := range color {
		if c == 0 {
			t0 = append(t0, k)
		} else {
			t1 = append(t1, k)
		}
	}
	sort.Ints(t0)
	sort.Ints(t1)
	return t0, t1, true
}

func group(g []int) string {
	parts := make([]string, len(g))
	for i, x := range g {
		parts[i] = strconv.Itoa(x)
	}
	return "{" + strings.Join(parts, ", ") + "}"
}

func main() {
	s1 := map[int][]int{0: {3}, 1: {2}, 2: {1, 4}, 3: {0, 4, 5}, 4: {2, 3}, 5: {3}}
	if t0, t1, ok := solve(s1); ok {
		fmt.Printf("%s and %s\n", group(t0), group(t1))
	} else {
		fmt.Println("False")
	}

	s2 := map[int][]int{0: {3}, 1: {2}, 2: {1, 3, 4}, 3: {0, 2, 4, 5}, 4: {2, 3}, 5: {3}}
	if t0, t1, ok := solve(s2); ok {
		fmt.Printf("%s and %s\n", group(t0), group(t1))
	} else {
		fmt.Println("False")
	}
}
