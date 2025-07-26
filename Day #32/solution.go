// Currency arbitrage: weights = -log(rate), Bellman-Ford detects negative cycle. O(V*E).
package main

import (
	"fmt"
	"math"
)

func hasArbitrage(rates [][]float64) bool {
	n := len(rates)
	w := make([][]float64, n)
	for i := range w {
		w[i] = make([]float64, n)
		for j := 0; j < n; j++ {
			w[i][j] = -math.Log(rates[i][j])
		}
	}
	dist := make([]float64, n) // virtual super-source reaching all nodes at 0
	for it := 0; it < n-1; it++ {
		for u := 0; u < n; u++ {
			for v := 0; v < n; v++ {
				if dist[u]+w[u][v] < dist[v]-1e-12 {
					dist[v] = dist[u] + w[u][v]
				}
			}
		}
	}
	for u := 0; u < n; u++ {
		for v := 0; v < n; v++ {
			if dist[u]+w[u][v] < dist[v]-1e-12 {
				return true
			}
		}
	}
	return false
}

func main() {
	r1 := [][]float64{{1.0, 0.7, 0.5}, {1.4, 1.0, 0.7}, {2.1, 1.4, 1.0}}
	r2 := [][]float64{{1.0, 2.0, 4.0}, {0.5, 1.0, 2.0}, {0.25, 0.5, 1.0}}
	fmt.Println(hasArbitrage(r1))
	fmt.Println(hasArbitrage(r2))
}
