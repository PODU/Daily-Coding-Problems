// Approximate median: take a small random sample (size s) and return its median.
// Rank lands in [N/4, 3N/4] w.h.p. Time O(s log s) = sub-linear, Space O(s).
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

func approxMedian(a []int, sampleSize int, rng *rand.Rand) int {
	n := len(a)
	s := sampleSize
	if s > n {
		s = n
	}
	sample := make([]int, s)
	for i := 0; i < s; i++ {
		sample[i] = a[rng.Intn(n)]
	}
	sort.Ints(sample)
	return sample[s/2]
}

func main() {
	rng := rand.New(rand.NewSource(42))
	n := 1000
	a := make([]int, n)
	for i := range a {
		a[i] = i
	}
	rng.Shuffle(n, func(i, j int) { a[i], a[j] = a[j], a[i] })

	m := approxMedian(a, 51, rng)
	rank := 0
	for _, x := range a {
		if x < m {
			rank++
		}
	}
	fmt.Println("approx median =", m)
	ok := rank >= n/4 && rank <= 3*n/4
	fmt.Printf("rank %d in [%d, %d]: %t\n", rank, n/4, 3*n/4, ok)
}
