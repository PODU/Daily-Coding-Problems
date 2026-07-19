// Day 1841: Top-k most similar website pairs by Jaccard similarity of their visitor sets.
// Time O(W^2 * U) over W websites; Space O(total pairs).
package main

import (
	"fmt"
	"sort"
	"strings"
)

type scoredPair struct {
	jac  float64
	a, b string
}

func main() {
	type vp struct {
		w string
		u int
	}
	pairs := []vp{
		{"a", 1}, {"a", 3}, {"a", 5},
		{"b", 2}, {"b", 6},
		{"c", 1}, {"c", 2}, {"c", 3}, {"c", 4}, {"c", 5},
		{"d", 4}, {"d", 5}, {"d", 6}, {"d", 7},
		{"e", 1}, {"e", 3}, {"e", 5}, {"e", 6},
	}
	k := 1

	sites := map[string]map[int]bool{}
	for _, p := range pairs {
		if sites[p.w] == nil {
			sites[p.w] = map[int]bool{}
		}
		sites[p.w][p.u] = true
	}
	var names []string
	for w := range sites {
		names = append(names, w)
	}
	sort.Strings(names)

	var scored []scoredPair
	for i := 0; i < len(names); i++ {
		for j := i + 1; j < len(names); j++ {
			A, B := sites[names[i]], sites[names[j]]
			inter := 0
			for x := range A {
				if B[x] {
					inter++
				}
			}
			union := len(A) + len(B) - inter
			jac := 0.0
			if union > 0 {
				jac = float64(inter) / float64(union)
			}
			scored = append(scored, scoredPair{jac, names[i], names[j]})
		}
	}
	sort.SliceStable(scored, func(i, j int) bool { return scored[i].jac > scored[j].jac })

	var out []string
	for i := 0; i < k; i++ {
		out = append(out, fmt.Sprintf("('%s', '%s')", scored[i].a, scored[i].b))
	}
	fmt.Println("[" + strings.Join(out, ", ") + "]")
}
