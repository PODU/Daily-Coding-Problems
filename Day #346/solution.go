// Day 346: Cheapest itinerary with up to k connections.
// Bellman-Ford limited to k+1 edges, tracking the path. Time O(k*E), Space O(V).
package main

import (
	"fmt"
	"strings"
)

type entry struct {
	cost int
	path []string
}

type flight struct {
	u, v string
	w    int
}

func cheapest(flights []flight, src, dst string, k int) (entry, bool) {
	best := map[string]entry{src: {0, []string{src}}}
	for it := 0; it <= k; it++ {
		nxt := make(map[string]entry, len(best))
		for key, val := range best {
			nxt[key] = val
		}
		for _, f := range flights {
			pu, ok := best[f.u]
			if !ok {
				continue
			}
			cost := pu.cost + f.w
			if cur, ok := nxt[f.v]; !ok || cost < cur.cost {
				np := append(append([]string{}, pu.path...), f.v)
				nxt[f.v] = entry{cost, np}
			}
		}
		best = nxt
	}
	e, ok := best[dst]
	return e, ok
}

func main() {
	flights := []flight{
		{"JFK", "ATL", 150}, {"ATL", "SFO", 400}, {"ORD", "LAX", 200},
		{"LAX", "DFW", 80}, {"JFK", "HKG", 800}, {"ATL", "ORD", 90}, {"JFK", "LAX", 500},
	}
	res, _ := cheapest(flights, "JFK", "LAX", 3)
	fmt.Printf("%s, costing $%d.\n", strings.Join(res.path, " -> "), res.cost)
}
