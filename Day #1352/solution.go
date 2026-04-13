// Arbitrage detection: edge weight = -log(rate); negative cycle => arbitrage. Bellman-Ford.
// Time: O(V*E) = O(V^3), Space: O(V).
package main

import (
	"fmt"
	"math"
)

func hasArbitrage(rates [][]float64) bool {
	n := len(rates)
	dist := make([]float64, n) // all 0: detect any reachable negative cycle
	for k := 0; k < n-1; k++ {
		for u := 0; u < n; u++ {
			for v := 0; v < n; v++ {
				w := -math.Log(rates[u][v])
				if dist[u]+w < dist[v]-1e-12 {
					dist[v] = dist[u] + w
				}
			}
		}
	}
	for u := 0; u < n; u++ {
		for v := 0; v < n; v++ {
			w := -math.Log(rates[u][v])
			if dist[u]+w < dist[v]-1e-12 {
				return true
			}
		}
	}
	return false
}

func main() {
	rates := [][]float64{
		{1.0, 2.0, 1.0},
		{0.5, 1.0, 4.0},
		{1.0, 0.25, 1.0},
	}
	fmt.Printf("Arbitrage exists: %t\n", hasArbitrage(rates))
}
