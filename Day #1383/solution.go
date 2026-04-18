// Weighted sampling: build cumulative prefix array, draw u in [0,1), sort.Search to pick. O(n) prep, O(log n) per sample.
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

func main() {
	numbers := []int{1, 2, 3, 4}
	probs := []float64{0.1, 0.5, 0.2, 0.2}
	n := len(numbers)

	cum := make([]float64, n)
	acc := 0.0
	for i, p := range probs {
		acc += p
		cum[i] = acc
	}

	rng := rand.New(rand.NewSource(42))
	const N = 100000
	counts := make([]int64, n)
	for s := 0; s < N; s++ {
		u := rng.Float64()
		idx := sort.Search(n, func(i int) bool { return cum[i] >= u })
		if idx >= n {
			idx = n - 1
		}
		counts[idx]++
	}

	for i := 0; i < n; i++ {
		fmt.Printf("%d: %.2f\n", numbers[i], float64(counts[i])/float64(N))
	}
}
