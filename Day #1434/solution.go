// Day 1434: Cheapest flight A->B with at most k connections; print itinerary.
// Approach: Bellman-Ford relaxed k+1 times (k connections = k+1 edges), track parent.
// Time: O((k+1) * E), Space: O(V).
package main

import (
	"fmt"
	"math"
	"strings"
)

type flight struct {
	src, dst string
	price    int
}

func cheapest(flights []flight, A, B string, k int) string {
	dist := map[string]int{}
	for _, f := range flights {
		if _, ok := dist[f.src]; !ok {
			dist[f.src] = math.MaxInt32
		}
		if _, ok := dist[f.dst]; !ok {
			dist[f.dst] = math.MaxInt32
		}
	}
	dist[A] = 0
	parent := map[string]string{}
	for i := 0; i <= k; i++ {
		cur := map[string]int{}
		for key, v := range dist {
			cur[key] = v
		}
		curParent := map[string]string{}
		for key, v := range parent {
			curParent[key] = v
		}
		for _, f := range flights {
			if dist[f.src] != math.MaxInt32 && dist[f.src]+f.price < cur[f.dst] {
				cur[f.dst] = dist[f.src] + f.price
				curParent[f.dst] = f.src
			}
		}
		dist, parent = cur, curParent
	}
	if dist[B] == math.MaxInt32 {
		return "No route"
	}
	var path []string
	node := B
	for node != A {
		path = append([]string{node}, path...)
		node = parent[node]
	}
	path = append([]string{A}, path...)
	return strings.Join(path, " -> ") + fmt.Sprintf(", costing $%d", dist[B])
}

func main() {
	flights := []flight{
		{"JFK", "ATL", 150}, {"ATL", "SFO", 400}, {"ORD", "LAX", 200},
		{"LAX", "DFW", 80}, {"JFK", "HKG", 800}, {"ATL", "ORD", 90}, {"JFK", "LAX", 500},
	}
	fmt.Println(cheapest(flights, "JFK", "LAX", 3))
}
