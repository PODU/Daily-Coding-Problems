// Day 495: Reservoir sampling (size 1) from a stream of unknown length.
// For the i-th element (1-indexed) keep it with probability 1/i. Uses O(1) memory.
// Time: O(n) per pass, Space: O(1).
package main

import (
	"fmt"
	"math/rand"
)

// Processes a stream via a channel without storing it; rng provides uniform doubles.
func reservoirSample(stream <-chan int, rng *rand.Rand) int {
	chosen, count := 0, 0
	for x := range stream {
		count++
		if rng.Float64() < 1.0/float64(count) {
			chosen = x
		}
	}
	return chosen
}

func makeStream(lo, hi int) <-chan int {
	ch := make(chan int)
	go func() {
		for i := lo; i <= hi; i++ {
			ch <- i
		}
		close(ch)
	}()
	return ch
}

func main() {
	rng := rand.New(rand.NewSource(42))
	const TRIALS = 100000
	counts := map[int]int{}
	for t := 0; t < TRIALS; t++ {
		counts[reservoirSample(makeStream(1, 10), rng)]++
	}

	fmt.Println("Empirical selection frequency per element (~0.100 each):")
	for v := 1; v <= 10; v++ {
		fmt.Printf("%d: %.3f\n", v, float64(counts[v])/float64(TRIALS))
	}
	fmt.Printf("One sampled value: %d\n", reservoirSample(makeStream(1, 10), rng))
}
