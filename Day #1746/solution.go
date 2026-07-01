// Day 1746: Weighted random sampler.
// Approach: prefix-sum (CDF) of probabilities + binary search on uniform U[0,1).
// Build O(n), sample O(log n) time, O(n) space.
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

type WeightedSampler struct {
	nums []int
	cdf  []float64
	rng  *rand.Rand
}

func NewSampler(nums []int, probs []float64, seed int64) *WeightedSampler {
	cdf := make([]float64, len(probs))
	acc := 0.0
	for i, p := range probs {
		acc += p
		cdf[i] = acc
	}
	return &WeightedSampler{nums, cdf, rand.New(rand.NewSource(seed))}
}

func (s *WeightedSampler) Sample() int {
	r := s.rng.Float64()
	idx := sort.SearchFloat64s(s.cdf, r)
	if idx >= len(s.nums) {
		idx = len(s.nums) - 1
	}
	return s.nums[idx]
}

func main() {
	numbers := []int{1, 2, 3, 4}
	probs := []float64{0.1, 0.5, 0.2, 0.2}
	s := NewSampler(numbers, probs, 42)

	N := 1000000
	cnt := make(map[int]int)
	for i := 0; i < N; i++ {
		cnt[s.Sample()]++
	}
	fmt.Printf("Observed frequencies over %d samples:\n", N)
	for _, x := range numbers {
		fmt.Printf("%d: %.1f%%\n", x, 100.0*float64(cnt[x])/float64(N))
	}
	fmt.Println("Expected: 1 10% of the time, 2 50% of the time, and 3 and 4 20% of the time")
}
