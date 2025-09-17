// Bipartite check (2-coloring) via BFS over enemy graph; split into two teams or False.
// Time O(V+E), Space O(V).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func twoColor(g map[int][]int, n int) []int {
	color := make([]int, n)
	for i := range color {
		color[i] = -1
	}
	for s := 0; s < n; s++ {
		if color[s] != -1 {
			continue
		}
		color[s] = 0
		q := []int{s}
		for len(q) > 0 {
			u := q[0]
			q = q[1:]
			for _, v := range g[u] {
				if color[v] == -1 {
					color[v] = color[u] ^ 1
					q = append(q, v)
				} else if color[v] == color[u] {
					return nil
				}
			}
		}
	}
	return color
}

func fmtSet(s []int) string {
	parts := make([]string, len(s))
	for i, x := range s {
		parts[i] = strconv.Itoa(x)
	}
	return "{" + strings.Join(parts, ", ") + "}"
}

func solve(g map[int][]int, n int) {
	color := twoColor(g, n)
	if color == nil {
		fmt.Println("False")
		return
	}
	var a, b []int
	for i := 0; i < n; i++ {
		if color[i] == 0 {
			a = append(a, i)
		} else {
			b = append(b, i)
		}
	}
	fmt.Printf("%s and %s\n", fmtSet(a), fmtSet(b))
}

func main() {
	g1 := map[int][]int{0: {3}, 1: {2}, 2: {1, 4}, 3: {0, 4, 5}, 4: {2, 3}, 5: {3}}
	g2 := map[int][]int{0: {3}, 1: {2}, 2: {1, 3, 4}, 3: {0, 2, 4, 5}, 4: {2, 3}, 5: {3}}
	solve(g1, 6)
	solve(g2, 6)
}
