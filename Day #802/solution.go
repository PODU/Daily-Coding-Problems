// Day 802: Sample a number per given probability distribution.
// Build CDF prefix sums once O(n); each sample draws u~U(0,1) + binary search O(log n).
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

func NewWeightedSampler(nums []int, probs []float64, seed int64) *WeightedSampler {
	cdf := make([]float64, len(probs))
	acc := 0.0
	for i, p := range probs {
		acc += p
		cdf[i] = acc
	}
	return &WeightedSampler{nums, cdf, rand.New(rand.NewSource(seed))}
}

func (s *WeightedSampler) Sample() int {
	u := s.rng.Float64()
	i := sort.SearchFloat64s(s.cdf, u)
	if i >= len(s.nums) {
		i = len(s.nums) - 1
	}
	return s.nums[i]
}

func main() {
	numbers := []int{1, 2, 3, 4}
	probs := []float64{0.1, 0.5, 0.2, 0.2}
	s := NewWeightedSampler(numbers, probs, 42)
	trials := 100000
	count := map[int]int{}
	for i := 0; i < trials; i++ {
		count[s.Sample()]++
	}
	for _, n := range numbers {
		fmt.Printf("%d: %.2f\n", n, float64(count[n])/float64(trials))
	}
	// ~ 1:0.10  2:0.50  3:0.20  4:0.20
}
