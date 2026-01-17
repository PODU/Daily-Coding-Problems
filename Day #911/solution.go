// Reservoir sampling (size 1): replace pick with prob 1/i for i-th element. O(n) time, O(1) space.
package main

import (
	"fmt"
	"math/rand"
)

func reservoirSample(stream []int, rng *rand.Rand) int {
	pick := 0
	for i, x := range stream {
		// (i+1) is 1-indexed position; replace with probability 1/(i+1)
		if rng.Intn(i+1) == 0 {
			pick = x
		}
	}
	return pick
}

func main() {
	rng := rand.New(rand.NewSource(12345))
	n := 10
	stream := make([]int, n)
	for k := range stream {
		stream[k] = k // 0..9
	}

	fmt.Println("Sampled element:", reservoirSample(stream, rng))

	trials := 100000
	freq := make([]int, n)
	for t := 0; t < trials; t++ {
		freq[reservoirSample(stream, rng)]++
	}
	fmt.Printf("Approx frequencies over %d trials (expect ~%.4f each):\n", trials, 1.0/float64(n))
	for v := 0; v < n; v++ {
		fmt.Printf("  %d: %.4f\n", v, float64(freq[v])/float64(trials))
	}
	fmt.Println("Distribution is ~uniform.")
}
