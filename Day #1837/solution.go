// Day 1837: Stable marriage via Gale-Shapley (men propose). Always yields a stable matching.
// Time O(N^2), Space O(N^2) for preference/ranking tables.
package main

import (
	"fmt"
	"sort"
)

func stableMatch(guyPref, galPref map[string][]string) map[string]string {
	galRank := map[string]map[string]int{}
	for gal, pref := range galPref {
		galRank[gal] = map[string]int{}
		for i, g := range pref {
			galRank[gal][g] = i
		}
	}
	next := map[string]int{}
	galPartner := map[string]string{}
	var free []string
	for guy := range guyPref {
		free = append(free, guy)
		next[guy] = 0
	}
	for len(free) > 0 {
		guy := free[0]
		free = free[1:]
		gal := guyPref[guy][next[guy]]
		next[guy]++
		cur, taken := galPartner[gal]
		if !taken {
			galPartner[gal] = guy
		} else if galRank[gal][guy] < galRank[gal][cur] {
			galPartner[gal] = guy
			free = append(free, cur)
		} else {
			free = append(free, guy)
		}
	}
	guyPartner := map[string]string{}
	for gal, guy := range galPartner {
		guyPartner[guy] = gal
	}
	return guyPartner
}

func main() {
	guyPref := map[string][]string{
		"andrew":  {"caroline", "abigail", "betty"},
		"bill":    {"caroline", "betty", "abigail"},
		"chester": {"betty", "caroline", "abigail"},
	}
	galPref := map[string][]string{
		"abigail":  {"andrew", "bill", "chester"},
		"betty":    {"bill", "andrew", "chester"},
		"caroline": {"bill", "chester", "andrew"},
	}
	match := stableMatch(guyPref, galPref)
	guys := make([]string, 0, len(match))
	for g := range match {
		guys = append(guys, g)
	}
	sort.Strings(guys)
	for _, g := range guys {
		fmt.Printf("%s -> %s\n", g, match[g])
	}
}
