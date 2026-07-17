// Arbitrage = negative cycle in graph with weights -log(rate). Bellman-Ford.
// O(V^3) for a dense rate table.
package main

import (
	"fmt"
	"math"
)

func hasArbitrage(rate [][]float64) bool {
	n := len(rate)
	w := make([][]float64, n)
	for i := range w {
		w[i] = make([]float64, n)
		for j := 0; j < n; j++ {
			w[i][j] = -math.Log(rate[i][j])
		}
	}
	dist := make([]float64, n) // virtual source, all 0
	for it := 0; it < n-1; it++ {
		for u := 0; u < n; u++ {
			for v := 0; v < n; v++ {
				if dist[u]+w[u][v] < dist[v] {
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
	rate := [][]float64{
		{1.0, 0.8, 0.5},
		{1.3, 1.0, 1.9},
		{1.9, 0.5, 1.0},
	}
	if hasArbitrage(rate) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
