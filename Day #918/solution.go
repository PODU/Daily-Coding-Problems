// PageRank via power iteration. Dangling nodes distribute rank evenly.
// Time O(iters * (N+E)), Space O(N+E).
package main

import (
	"fmt"
	"math"
)

func pagerank(out [][]int, d float64, maxIter int, tol float64) []float64 {
	N := len(out)
	rank := make([]float64, N)
	for i := range rank {
		rank[i] = 1.0 / float64(N)
	}
	for iter := 0; iter < maxIter; iter++ {
		next := make([]float64, N)
		for j := range next {
			next[j] = (1.0 - d) / float64(N)
		}
		dangling := 0.0
		for i := 0; i < N; i++ {
			if len(out[i]) == 0 {
				dangling += rank[i]
			}
		}
		for i := 0; i < N; i++ {
			if len(out[i]) == 0 {
				continue
			}
			share := rank[i] / float64(len(out[i]))
			for _, j := range out[i] {
				next[j] += d * share
			}
		}
		for j := 0; j < N; j++ {
			next[j] += d * dangling / float64(N)
		}
		diff := 0.0
		for j := 0; j < N; j++ {
			diff += math.Abs(next[j] - rank[j])
		}
		rank = next
		if diff < tol {
			break
		}
	}
	return rank
}

func main() {
	out := [][]int{{1, 2}, {2}, {0, 1}, {0, 1, 2}}
	rank := pagerank(out, 0.85, 1000, 1e-9)
	for i, r := range rank {
		fmt.Printf("%d: %.4f\n", i, r)
	}
}
