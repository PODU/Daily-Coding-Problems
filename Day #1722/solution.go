// Day 1722: Approximate median via random sampling.
// Sample a sublinear number of elements (~constant), return their exact median.
// With high probability its rank lies in [N/4, 3N/4]. Time: O(s log s) << O(N), Space: O(s).
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

func approxMedian(a []int, rng *rand.Rand) int {
	n := len(a)
	s := n
	if s > 99 {
		s = 99 // sublinear sample size
	}
	sample := make([]int, s)
	for i := 0; i < s; i++ {
		sample[i] = a[rng.Intn(n)]
	}
	sort.Ints(sample)
	return sample[s/2]
}

func main() {
	// Demo: values 0..99 shuffled deterministically.
	N := 100
	a := make([]int, N)
	for i := range a {
		a[i] = i
	}
	rng := rand.New(rand.NewSource(42))
	rng.Shuffle(N, func(i, j int) { a[i], a[j] = a[j], a[i] })

	m := approxMedian(a, rng)
	rank := 0
	for _, x := range a {
		if x < m {
			rank++
		}
	}
	fmt.Printf("Approximate median: %d (rank %d within [N/4, 3N/4] = [%d, %d])\n",
		m, rank, N/4, 3*N/4)
}
