// Day 1214: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian path over sorted multigraph adjacency. Time O(E log E), Space O(E).
package main

import (
	"fmt"
	"sort"
)

func findItinerary(flights [][2]string, start string) []string {
	adj := map[string][]string{}
	for _, f := range flights {
		adj[f[0]] = append(adj[f[0]], f[1])
	}
	for k := range adj {
		sort.Sort(sort.Reverse(sort.StringSlice(adj[k]))) // pop from end = smallest
	}
	total := len(flights)
	var route []string
	st := []string{start}
	for len(st) > 0 {
		u := st[len(st)-1]
		if len(adj[u]) > 0 {
			n := len(adj[u])
			v := adj[u][n-1]
			adj[u] = adj[u][:n-1]
			st = append(st, v)
		} else {
			route = append(route, u)
			st = st[:len(st)-1]
		}
	}
	for l, r := 0, len(route)-1; l < r; l, r = l+1, r-1 {
		route[l], route[r] = route[r], route[l]
	}
	if len(route) != total+1 {
		return nil
	}
	return route
}

func main() {
	flights := [][2]string{{"SFO", "HKO"}, {"YYZ", "SFO"}, {"YUL", "YYZ"}, {"HKO", "ORD"}}
	r := findItinerary(flights, "YUL")
	fmt.Printf("%q\n", r) // ["YUL" "YYZ" "SFO" "HKO" "ORD"]
}
