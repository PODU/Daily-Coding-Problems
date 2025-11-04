// Arbitrage detection: weight=-log(rate), Bellman-Ford finds a negative-weight cycle => arbitrage. O(V^3) (V*E edges, V-1 passes).
package main

import (
	"fmt"
	"math"
)

func hasArbitrage(rates [][]float64) bool {
	n := len(rates)
	dist := make([]float64, n) // virtual source: all start at 0
	for it := 0; it < n-1; it++ {
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
	arb := [][]float64{{1, 0.5, 0.2}, {2, 1, 0.5}, {5, 2, 1}}
	consistent := [][]float64{{1, 2, 4}, {0.5, 1, 2}, {0.25, 0.5, 1}}
	if hasArbitrage(arb) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
	if hasArbitrage(consistent) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
