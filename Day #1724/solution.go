// Day 1724: Cheapest itinerary with at most k connections (k flights/edges).
// Bellman-Ford limited to k relaxation rounds; track parents to reconstruct path.
// Time: O(k * E), Space: O(V).
package main

import (
	"fmt"
	"strings"
)

type flight struct {
	u, v string
	w    int
}

func main() {
	flights := []flight{
		{"JFK", "ATL", 150}, {"ATL", "SFO", 400}, {"ORD", "LAX", 200},
		{"LAX", "DFW", 80}, {"JFK", "HKG", 800}, {"ATL", "ORD", 90},
		{"JFK", "LAX", 500},
	}
	src, dst, k := "JFK", "LAX", 3

	const INF = int(^uint(0) >> 1)
	dist := map[string]int{}
	parent := map[string]string{}
	for _, f := range flights {
		if _, ok := dist[f.u]; !ok {
			dist[f.u] = INF
		}
		if _, ok := dist[f.v]; !ok {
			dist[f.v] = INF
		}
	}
	dist[src] = 0

	// Relax all edges k times over a snapshot to bound edge count.
	for i := 0; i < k; i++ {
		snap := map[string]int{}
		for kk, vv := range dist {
			snap[kk] = vv
		}
		for _, f := range flights {
			if snap[f.u] != INF && snap[f.u]+f.w < dist[f.v] {
				dist[f.v] = snap[f.u] + f.w
				parent[f.v] = f.u
			}
		}
	}

	if dist[dst] == INF {
		fmt.Println("No route")
		return
	}
	var path []string
	for at := dst; ; at = parent[at] {
		path = append([]string{at}, path...)
		if at == src {
			break
		}
	}
	fmt.Printf("%s, costing $%d\n", strings.Join(path, " -> "), dist[dst])
}
