// Group users into a set per site; for every site pair compute Jaccard = |A&B|/|A|B|.
// Sort by similarity DESC, tie-break by pair lexicographically; take top k. Time O(P^2 * U).
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

type cand struct {
	sim  float64
	x, y string
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
	for s := range sites {
		names = append(names, s)
	}
	sort.Strings(names)

	var cands []cand
	for i := 0; i < len(names); i++ {
		for j := i + 1; j < len(names); j++ {
			a := sites[names[i]]
			b := sites[names[j]]
			inter := 0
			for u := range a {
				if b[u] {
					inter++
				}
			}
			uni := len(a) + len(b) - inter
			sim := 0.0
			if uni != 0 {
				sim = float64(inter) / float64(uni)
			}
			cands = append(cands, cand{sim, names[i], names[j]})
		}
	}
	sort.SliceStable(cands, func(i, j int) bool {
		if cands[i].sim != cands[j].sim {
			return cands[i].sim > cands[j].sim
		}
		if cands[i].x != cands[j].x {
			return cands[i].x < cands[j].x
		}
		return cands[i].y < cands[j].y
	})

	var parts []string
	for i := 0; i < k && i < len(cands); i++ {
		parts = append(parts, fmt.Sprintf("('%s', '%s')", cands[i].x, cands[i].y))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
