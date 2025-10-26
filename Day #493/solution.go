// Day 493: Sample from a discrete distribution.
// Precompute cumulative probabilities; binary-search a uniform r in [0,1).
// Time: O(n) preprocessing, O(log n) per sample. Space: O(n).
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

type DiscreteSampler struct {
	numbers []int
	cdf     []float64
}

func NewDiscreteSampler(numbers []int, probs []float64) *DiscreteSampler {
	cdf := make([]float64, len(probs))
	acc := 0.0
	for i, p := range probs {
		acc += p
		cdf[i] = acc
	}
	return &DiscreteSampler{numbers, cdf}
}

func (s *DiscreteSampler) Sample(r float64) int {
	// first index whose cdf > r
	idx := sort.Search(len(s.cdf), func(i int) bool { return s.cdf[i] > r })
	if idx >= len(s.numbers) {
		idx = len(s.numbers) - 1
	}
	return s.numbers[idx]
}

func main() {
	numbers := []int{1, 2, 3, 4}
	probs := []float64{0.1, 0.5, 0.2, 0.2}
	s := NewDiscreteSampler(numbers, probs)

	rng := rand.New(rand.NewSource(42))
	const N = 100000
	counts := map[int]int{}
	for i := 0; i < N; i++ {
		counts[s.Sample(rng.Float64())]++
	}
	for _, n := range numbers {
		fmt.Printf("%d: %.3f\n", n, float64(counts[n])/float64(N))
	}
}
