// Day 1213: Stable marriage via Gale-Shapley (men propose).
// Each free man proposes down his list; women keep their best suitor. Time O(N^2), Space O(N^2).
package main

import (
	"fmt"
	"sort"
)

func stableMatch(guys, gals map[string][]string) map[string]string {
	rank := map[string]map[string]int{}
	for w, prefs := range gals {
		rank[w] = map[string]int{}
		for i, m := range prefs {
			rank[w][m] = i
		}
	}
	next := map[string]int{}
	engaged := map[string]string{} // woman -> man
	var free []string
	for m := range guys {
		free = append(free, m)
		next[m] = 0
	}
	for len(free) > 0 {
		m := free[0]
		free = free[1:]
		w := guys[m][next[m]]
		next[m]++
		if cur, ok := engaged[w]; !ok {
			engaged[w] = m
		} else if rank[w][m] < rank[w][cur] {
			engaged[w] = m
			free = append(free, cur)
		} else {
			free = append(free, m)
		}
	}
	return engaged
}

func main() {
	guys := map[string][]string{
		"andrew":  {"caroline", "abigail", "betty"},
		"bill":    {"caroline", "betty", "abigail"},
		"chester": {"betty", "caroline", "abigail"},
	}
	gals := map[string][]string{
		"abigail":  {"andrew", "bill", "chester"},
		"betty":    {"bill", "andrew", "chester"},
		"caroline": {"bill", "chester", "andrew"},
	}
	match := stableMatch(guys, gals)
	byMan := map[string]string{}
	for w, m := range match {
		byMan[m] = w
	}
	var men []string
	for m := range guys {
		men = append(men, m)
	}
	sort.Strings(men)
	for _, m := range men {
		fmt.Printf("%s - %s\n", m, byMan[m])
	}
	// andrew - abigail / bill - caroline / chester - betty
}
