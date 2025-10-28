// Approximate median: sample a fixed, sublinear subset (size independent of N),
// return the sample's median -> lands in the central half [N/4, 3N/4] w.h.p.
// Sampling+median: O(s log s), sublinear in N. (Rank shown only for the demo.)
package main

import (
	"fmt"
	"sort"
)

var state uint64 = 0x0123456789ABCDEF // fixed seed -> deterministic

func nextRand() uint64 {
	state = state*6364136223846793005 + 1442695040888963407 // wraps mod 2^64
	return state >> 33                                      // top 31 bits
}

func main() {
	const N = 1000
	const SampleSize = 99 // fixed, independent of N (sublinear)

	data := make([]int, N)
	for i := 0; i < N; i++ {
		data[i] = i + 1
	}
	for i := N - 1; i > 0; i-- {
		j := int(nextRand() % uint64(i+1))
		data[i], data[j] = data[j], data[i]
	}

	sample := make([]int, 0, SampleSize)
	for i := 0; i < SampleSize; i++ {
		idx := int(nextRand() % uint64(N))
		sample = append(sample, data[idx])
	}
	sort.Ints(sample)
	median := sample[SampleSize/2]

	rank := 0
	for _, v := range data {
		if v <= median {
			rank++
		}
	}

	fmt.Printf("Approximate median: %d\n", median)
	fmt.Printf("Rank: %d (acceptable range: %d to %d)\n", rank, N/4, 3*N/4)
}
