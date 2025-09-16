// Top-k similar website pairs by Jaccard similarity of user sets.
// Build per-site user sets, compute Jaccard for all pairs, sort desc (ties alpha), take k.
// Time: O(S^2 * U), Space: O(S*U).
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

type result struct {
	sim  float64
	a, b string
}

func main() {
	visits := []visit{
		{"a", 1}, {"a", 3}, {"a", 5}, {"b", 2}, {"b", 6},
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

	var results []result
	for i := 0; i < len(names); i++ {
		for j := i + 1; j < len(names); j++ {
			A, B := sites[names[i]], sites[names[j]]
			inter := 0
			for x := range A {
				if B[x] {
					inter++
				}
			}
			uni := len(A) + len(B) - inter
			sim := 0.0
			if uni != 0 {
				sim = float64(inter) / float64(uni)
			}
			results = append(results, result{sim, names[i], names[j]})
		}
	}
	sort.Slice(results, func(i, j int) bool {
		if results[i].sim != results[j].sim {
			return results[i].sim > results[j].sim
		}
		if results[i].a != results[j].a {
			return results[i].a < results[j].a
		}
		return results[i].b < results[j].b
	})

	var parts []string
	for i := 0; i < k && i < len(results); i++ {
		parts = append(parts, fmt.Sprintf("('%s', '%s')", results[i].a, results[i].b))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
