// Day 941: Arbitrage exists iff a cycle has product of rates > 1, i.e. a negative cycle
// under weights w = -log(rate). Bellman-Ford. Time O(V^3), Space O(V).
package main

import (
	"fmt"
	"math"
)

func hasArbitrage(rate [][]float64) bool {
	n := len(rate)
	dist := make([]float64, n) // virtual source: all 0
	for iter := 0; iter < n-1; iter++ {
		for u := 0; u < n; u++ {
			for v := 0; v < n; v++ {
				w := -math.Log(rate[u][v])
				if dist[u]+w < dist[v]-1e-12 {
					dist[v] = dist[u] + w
				}
			}
		}
	}
	for u := 0; u < n; u++ {
		for v := 0; v < n; v++ {
			w := -math.Log(rate[u][v])
			if dist[u]+w < dist[v]-1e-12 {
				return true
			}
		}
	}
	return false
}

func main() {
	rate := [][]float64{
		{1.0, 2.0, 1.0},
		{0.5, 1.0, 2.0},
		{1.0, 0.5, 1.0},
	}
	fmt.Println(hasArbitrage(rate)) // true (0->1->2->0 = 4 > 1)
}
