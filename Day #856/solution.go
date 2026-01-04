// Day 856: Top k similar website pairs.
// Approach: Jaccard similarity (|A∩B| / |A∪B|) over user sets, take top k pairs.
// Time: O(W^2 * U), Space: O(W * U).
package main

import (
	"fmt"
	"sort"
	"strings"
)

type visit struct {
	site string
	user int
}

func main() {
	visits := []visit{
		{"a", 1}, {"a", 3}, {"a", 5},
		{"b", 2}, {"b", 6},
		{"c", 1}, {"c", 2}, {"c", 3}, {"c", 4}, {"c", 5},
		{"d", 4}, {"d", 5}, {"d", 6}, {"d", 7},
		{"e", 1}, {"e", 3}, {"e", 5}, {"e", 6},
	}
	k := 1

	sites := map[string]map[int]bool{}
	for _, v := range visits {
		if sites[v.site] == nil {
			sites[v.site] = map[int]bool{}
		}
		sites[v.site][v.user] = true
	}
	var names []string
	for n := range sites {
		names = append(names, n)
	}
	sort.Strings(names)

	type scoredPair struct {
		sim  float64
		a, b string
	}
	var scored []scoredPair
	for i := 0; i < len(names); i++ {
		for j := i + 1; j < len(names); j++ {
			A, B := sites[names[i]], sites[names[j]]
			inter := 0
			for u := range A {
				if B[u] {
					inter++
				}
			}
			uni := len(A) + len(B) - inter
			sim := 0.0
			if uni > 0 {
				sim = float64(inter) / float64(uni)
			}
			scored = append(scored, scoredPair{sim, names[i], names[j]})
		}
	}
	sort.SliceStable(scored, func(i, j int) bool { return scored[i].sim > scored[j].sim })

	var parts []string
	for i := 0; i < k && i < len(scored); i++ {
		parts = append(parts, fmt.Sprintf("('%s', '%s')", scored[i].a, scored[i].b))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
