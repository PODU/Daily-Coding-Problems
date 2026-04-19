// Top-k website pairs by Jaccard similarity over user sets; sort desc, tie-break alpha.
// O(W^2 * U) to compare pairs. Output formatted as Python tuple list.
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
	a, b string
	sim  float64
}

func main() {
	visits := []visit{
		{"a", 1}, {"a", 3}, {"a", 5}, {"b", 2}, {"b", 6},
		{"c", 1}, {"c", 2}, {"c", 3}, {"c", 4}, {"c", 5},
		{"d", 4}, {"d", 5}, {"d", 6}, {"d", 7},
		{"e", 1}, {"e", 3}, {"e", 5}, {"e", 6},
	}
	k := 1

	users := map[string]map[int]bool{}
	for _, v := range visits {
		if users[v.site] == nil {
			users[v.site] = map[int]bool{}
		}
		users[v.site][v.user] = true
	}

	var sites []string
	for s := range users {
		sites = append(sites, s)
	}
	sort.Strings(sites)

	var results []result
	for i := 0; i < len(sites); i++ {
		for j := i + 1; j < len(sites); j++ {
			A, B := users[sites[i]], users[sites[j]]
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
			results = append(results, result{sites[i], sites[j], sim})
		}
	}

	sort.SliceStable(results, func(x, y int) bool {
		if results[x].sim != results[y].sim {
			return results[x].sim > results[y].sim
		}
		if results[x].a != results[y].a {
			return results[x].a < results[y].a
		}
		return results[x].b < results[y].b
	})

	var parts []string
	for i := 0; i < k && i < len(results); i++ {
		parts = append(parts, fmt.Sprintf("('%s', '%s')", results[i].a, results[i].b))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
