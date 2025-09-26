// PageRank via iterative power method, d=0.85. Iterate until convergence.
// Time: O(iters * (N + E)). Space: O(N + E).
package main

import (
	"fmt"
	"math"
)

func main() {
	names := []string{"A", "B", "C", "D"}
	N := len(names)
	idx := map[string]int{}
	for i, n := range names {
		idx[n] = i
	}
	edges := [][2]string{{"A", "B"}, {"A", "C"}, {"B", "C"}, {"C", "A"}, {"D", "C"}}
	incoming := make([][]int, N)
	out := make([]int, N)
	for _, e := range edges {
		incoming[idx[e[1]]] = append(incoming[idx[e[1]]], idx[e[0]])
		out[idx[e[0]]]++
	}

	d := 0.85
	score := make([]float64, N)
	for i := range score {
		score[i] = 1.0 / float64(N)
	}
	for it := 0; it < 1000; it++ {
		nxt := make([]float64, N)
		for j := 0; j < N; j++ {
			nxt[j] = (1.0 - d) / float64(N)
		}
		for j := 0; j < N; j++ {
			for _, i := range incoming[j] {
				nxt[j] += d * score[i] / float64(out[i])
			}
		}
		diff := 0.0
		for k := 0; k < N; k++ {
			diff += math.Abs(nxt[k] - score[k])
		}
		score = nxt
		if diff < 1e-9 {
			break
		}
	}
	for i := 0; i < N; i++ {
		fmt.Printf("%s: %.4f\n", names[i], score[i])
	}
}
