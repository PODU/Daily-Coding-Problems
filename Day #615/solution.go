// Stable Marriage (Gale-Shapley, men proposing). Each free man proposes down his list.
// Time: O(N^2), Space: O(N^2) for preference rank tables.
package main

import (
	"fmt"
	"strings"
)

func galeShapley(men []string, guy, gal map[string][]string) map[string]string {
	rank := map[string]map[string]int{}
	for w, prefs := range gal {
		rank[w] = map[string]int{}
		for i, m := range prefs {
			rank[w][m] = i
		}
	}
	next := map[string]int{}
	husband := map[string]string{}
	free := append([]string{}, men...)

	for len(free) > 0 {
		m := free[0]
		free = free[1:]
		w := guy[m][next[m]]
		next[m]++
		if _, ok := husband[w]; !ok {
			husband[w] = m
		} else {
			cur := husband[w]
			if rank[w][m] < rank[w][cur] {
				husband[w] = m
				free = append(free, cur)
			} else {
				free = append(free, m)
			}
		}
	}

	wife := map[string]string{}
	for w, m := range husband {
		wife[m] = w
	}
	res := map[string]string{}
	for _, m := range men {
		res[m] = wife[m]
	}
	return res
}

func main() {
	guy := map[string][]string{
		"andrew":  {"caroline", "abigail", "betty"},
		"bill":    {"caroline", "betty", "abigail"},
		"chester": {"betty", "caroline", "abigail"},
	}
	gal := map[string][]string{
		"abigail":  {"andrew", "bill", "chester"},
		"betty":    {"bill", "andrew", "chester"},
		"caroline": {"bill", "chester", "andrew"},
	}
	men := []string{"andrew", "bill", "chester"}
	res := galeShapley(men, guy, gal)

	parts := make([]string, 0, len(men))
	for _, m := range men {
		parts = append(parts, fmt.Sprintf("'%s': '%s'", m, res[m]))
	}
	fmt.Printf("{%s}\n", strings.Join(parts, ", "))
}
