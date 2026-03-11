// Approximate median: take a constant-size random sample and return its median.
// Sublinear: O(k log k) for constant k samples, independent of N. Space: O(k).
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

func approxMedian(a []int) (int, int) {
	N := len(a)
	k := 31
	if N < k {
		k = N
	}
	rng := rand.New(rand.NewSource(42)) // fixed seed for reproducibility
	sample := make([]int, k)
	for i := 0; i < k; i++ {
		sample[i] = a[rng.Intn(N)]
	}
	sort.Ints(sample)
	med := sample[k/2]
	rank := 0
	for _, x := range a {
		if x <= med {
			rank++
		}
	}
	return med, rank
}

func main() {
	N := 100
	a := make([]int, N)
	for i := 0; i < N; i++ {
		a[i] = i + 1
	}
	med, rank := approxMedian(a)
	fmt.Printf("Approximate median: %d (rank %d within [%d, %d])\n", med, rank, N/4, 3*N/4)
}
