// Day 1052: Graph bipartiteness / 2-coloring via BFS over all components.
// Time O(V+E), Space O(V). Returns two teams or False if not bipartite.

package main

import (
	"fmt"
	"sort"
	"strings"
)

func divideTeams(students map[int][]int) (bool, []int, []int) {
	color := map[int]int{}
	keys := make([]int, 0, len(students))
	for k := range students {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	for _, start := range keys {
		if _, ok := color[start]; ok {
			continue
		}
		color[start] = 0
		q := []int{start}
		for len(q) > 0 {
			u := q[0]
			q = q[1:]
			for _, v := range students[u] {
				if _, ok := color[v]; !ok {
					color[v] = color[u] ^ 1
					q = append(q, v)
				} else if color[v] == color[u] {
					return false, nil, nil
				}
			}
		}
	}
	var a, b []int
	for _, n := range keys {
		if color[n] == 0 {
			a = append(a, n)
		} else {
			b = append(b, n)
		}
	}
	sort.Ints(a)
	sort.Ints(b)
	return true, a, b
}

func setStr(v []int) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = fmt.Sprintf("%d", x)
	}
	return "{" + strings.Join(parts, ", ") + "}"
}

func fmtResult(ok bool, a, b []int) string {
	if !ok {
		return "False"
	}
	return setStr(a) + " and " + setStr(b)
}

func main() {
	s1 := map[int][]int{0: {3}, 1: {2}, 2: {1, 4}, 3: {0, 4, 5}, 4: {2, 3}, 5: {3}}
	s2 := map[int][]int{0: {3}, 1: {2}, 2: {1, 3, 4}, 3: {0, 2, 4, 5}, 4: {2, 3}, 5: {3}}
	ok, a, b := divideTeams(s1)
	fmt.Println(fmtResult(ok, a, b))
	ok, a, b = divideTeams(s2)
	fmt.Println(fmtResult(ok, a, b))
}
