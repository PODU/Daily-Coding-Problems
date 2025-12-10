// Day 728: Split students into two teams so no enemies share a team.
// Approach: BFS 2-coloring (bipartite check). Returns two teams or reports failure.
// Time: O(V + E), Space: O(V).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func twoTeams(students map[int][]int) ([]int, []int, bool) {
	color := make(map[int]int)
	keys := make([]int, 0, len(students))
	for k := range students {
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
			for _, v := range students[u] {
				if c, ok := color[v]; !ok {
					color[v] = color[u] ^ 1
					q = append(q, v)
				} else if c == color[u] {
					return nil, nil, false
				}
			}
		}
	}
	var a, b []int
	for _, k := range keys {
		if color[k] == 0 {
			a = append(a, k)
		} else {
			b = append(b, k)
		}
	}
	return a, b, true
}

func setStr(v []int) string {
	s := make([]string, len(v))
	for i, x := range v {
		s[i] = strconv.Itoa(x)
	}
	return "{" + strings.Join(s, ", ") + "}"
}

func show(students map[int][]int) {
	a, b, ok := twoTeams(students)
	if !ok {
		fmt.Println("False")
		return
	}
	fmt.Println(setStr(a) + " and " + setStr(b))
}

func main() {
	show(map[int][]int{0: {3}, 1: {2}, 2: {1, 4}, 3: {0, 4, 5}, 4: {2, 3}, 5: {3}})
	show(map[int][]int{0: {3}, 1: {2}, 2: {1, 3, 4}, 3: {0, 2, 4, 5}, 4: {2, 3}, 5: {3}})
}
