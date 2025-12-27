// Day 804: Cheapest fare from A to B with <= k connections (<= k+1 flights).
// Bellman-Ford limited to k+1 relaxation rounds, tracking parents for itinerary.
// Time O((k+1) * E), Space O(V + E).
package main

import (
	"fmt"
	"math"
	"strings"
)

type Flight struct {
	src, dst string
	price    int
}

func cheapest(flights []Flight, A, B string, k int) (int, []string) {
	const INF = math.MaxInt32
	dist := map[string]int{}
	for _, f := range flights {
		dist[f.src] = INF
		dist[f.dst] = INF
	}
	dist[A] = 0
	parent := map[string]string{}
	for it := 0; it <= k; it++ {
		snap := map[string]int{}
		for c, v := range dist {
			snap[c] = v
		}
		for _, f := range flights {
			if snap[f.src] == INF {
				continue
			}
			if snap[f.src]+f.price < dist[f.dst] {
				dist[f.dst] = snap[f.src] + f.price
				parent[f.dst] = f.src
			}
		}
	}
	if dist[B] == INF {
		return -1, nil
	}
	var path []string
	for c := B; ; c = parent[c] {
		path = append([]string{c}, path...)
		if c == A {
			break
		}
	}
	return dist[B], path
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
