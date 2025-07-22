// Reservoir sampling (k=1): keep current pick with prob 1/i as i-th item streams.
// Time: O(n) single pass, Space: O(1).
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

func pickRandom(stream []int, rng *rand.Rand) int {
	chosen, count := 0, 0
	for _, x := range stream {
		count++
		if rng.Intn(count) == 0 {
			chosen = x
		}
	}
	return chosen
}

func main() {
	rng := rand.New(rand.NewSource(42))
	stream := []int{10, 20, 30, 40, 50}
	freq := map[int]int{}
	for t := 0; t < 100000; t++ {
		freq[pickRandom(stream, rng)]++
	}
	fmt.Println("One sample:", pickRandom(stream, rng))
	fmt.Println("Approx frequencies over 100000 trials:")
	keys := []int{}
	for k := range freq {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	for _, k := range keys {
		fmt.Printf("  %d: %.3f\n", k, float64(freq[k])/100000.0)
	}
}
