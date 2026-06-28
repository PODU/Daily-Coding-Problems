// Day 1728: Simulate a fair coin from a biased one (Von Neumann trick).
// Toss biased coin twice; (0,1)->0, (1,0)->1, else retry. Expected P(heads) ~= 0.5.
// Time: O(1) expected tosses per fair() call. Space: O(1).
package main

import (
	"fmt"
	"math/rand"
)

var rng = rand.New(rand.NewSource(12345))

// tossBiased returns 1 with probability p (= 0.3), else 0.
func tossBiased() int {
	if rng.Float64() < 0.3 {
		return 1
	}
	return 0
}

// fair extracts a fair bit from the biased coin (Von Neumann).
func fair() int {
	for {
		a := tossBiased()
		b := tossBiased()
		if a == 0 && b == 1 {
			return 0
		}
		if a == 1 && b == 0 {
			return 1
		}
	}
}

func main() {
	const N = 100000
	heads := 0
	for i := 0; i < N; i++ {
		heads += fair()
	}
	ratio := float64(heads) / float64(N)
	fmt.Printf("Fair coin over %d tosses, P(heads) ~= %.2f\n", N, ratio)
}
