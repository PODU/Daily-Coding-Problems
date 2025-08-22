// Day 152: Weighted random sampling. Build cumulative distribution, draw u in
// [0,1) and binary-search the bucket. Preprocess O(n), per-sample O(log n).
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
	return &WeightedSampler{nums: nums, cdf: cdf, rng: rand.New(rand.NewSource(seed))}
}

func (s *WeightedSampler) Sample() int {
	u := s.rng.Float64()
	idx := sort.Search(len(s.cdf), func(i int) bool { return s.cdf[i] >= u })
	if idx >= len(s.nums) {
		idx = len(s.nums) - 1
	}
	return s.nums[idx]
}

func main() {
	nums := []int{1, 2, 3, 4}
	probs := []float64{0.1, 0.5, 0.2, 0.2}
	s := NewSampler(nums, probs, 42)
	N := 1000000
	counts := make(map[int]int)
	for i := 0; i < N; i++ {
		counts[s.Sample()]++
	}
	for _, n := range nums {
		fmt.Printf("%d: %.2f%%\n", n, 100.0*float64(counts[n])/float64(N))
	}
}
