// Approximate median: median of k random samples (seeded RNG) -> rank in [N/4, 3N/4] whp.
// Time: O(k log k), o(N) for k<N; Space: O(k).
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

func approxMedian(a []int, k int, seed int64) int {
	rng := rand.New(rand.NewSource(seed))
	sample := make([]int, k)
	for i := 0; i < k; i++ {
		sample[i] = a[rng.Intn(len(a))]
	}
	sort.Ints(sample)
	return sample[k/2]
}

func main() {
	a := make([]int, 101) // N = 101, values 0..100
	for i := 0; i <= 100; i++ {
		a[i] = i
	}
	N := len(a)
	val := approxMedian(a, 15, 42)
	rank := val // rank in sorted 0..100 equals value
	ok := N/4 <= rank && rank <= (3*N)/4
	result := "false"
	if ok {
		result = "true"
	}
	fmt.Printf("Approximate median is within [N/4, 3N/4]: %s\n", result)
}
