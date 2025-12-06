// Day 700: Cheapest itinerary with at most k connections (k+1 flights).
// Approach: Bellman-Ford bounded to k+1 edges (relax with previous layer's dist),
// track predecessors to rebuild the route. Time O((k+1)*E), Space O(V).
package main

import (
	"fmt"
	"strings"
)

type Flight struct {
	u, v string
	w    int
}

func cheapest(flights []Flight, src, dst string, k int) (int, []string) {
	dist := map[string]int{src: 0}
	par := map[string]string{}
	for it := 0; it <= k; it++ { // up to k+1 edges
		nd := map[string]int{}
		np := map[string]string{}
		for kk, vv := range dist {
			nd[kk] = vv
		}
		for kk, vv := range par {
			np[kk] = vv
		}
		for _, f := range flights {
			if du, ok := dist[f.u]; ok {
				cand := du + f.w
				if cur, ok2 := nd[f.v]; !ok2 || cand < cur {
					nd[f.v] = cand
					np[f.v] = f.u
				}
			}
		}
		dist, par = nd, np
	}
	if _, ok := dist[dst]; !ok {
		return -1, nil
	}
	var path []string
	cur := dst
	for cur != src {
		path = append([]string{cur}, path...)
		cur = par[cur]
	}
	path = append([]string{src}, path...)
	return dist[dst], path
}

func main() {
	flights := []Flight{
		{"JFK", "ATL", 150}, {"ATL", "SFO", 400}, {"ORD", "LAX", 200},
		{"LAX", "DFW", 80}, {"JFK", "HKG", 800}, {"ATL", "ORD", 90},
		{"JFK", "LAX", 500},
	}
	cost, path := cheapest(flights, "JFK", "LAX", 3)
	fmt.Printf("%s, costing $%d\n", strings.Join(path, " -> "), cost)
	// JFK -> ATL -> ORD -> LAX, costing $440
}
