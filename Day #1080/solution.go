// Markov chain simulation: seeded math/rand; O(steps*states) time O(states^2) space
// Counts state arrived at after each step (not initial state); total counts = num_steps = 5000
// Exact counts vary by seed; approx distribution: ~60% a, ~33% b, ~7% c
package main

import (
	"fmt"
	"math/rand"
)

func main() {
	type Trans struct {
		to   string
		prob float64
	}
	trans := map[string][]Trans{
		"a": {{"a", 0.9}, {"b", 0.075}, {"c", 0.025}},
		"b": {{"a", 0.15}, {"b", 0.8}, {"c", 0.05}},
		"c": {{"a", 0.25}, {"b", 0.25}, {"c", 0.5}},
	}

	rng := rand.New(rand.NewSource(42))
	state := "a"
	counts := map[string]int{"a": 0, "b": 0, "c": 0}
	const numSteps = 5000

	for i := 0; i < numSteps; i++ {
		r := rng.Float64()
		cumulative := 0.0
		for _, t := range trans[state] {
			cumulative += t.prob
			if r < cumulative {
				state = t.to
				break
			}
		}
		counts[state]++
	}

	fmt.Printf("{ 'a': %d, 'b': %d, 'c': %d }\n", counts["a"], counts["b"], counts["c"])
}
