// Markov chain simulation: sample next state via cumulative probabilities, fixed-seed RNG.
// Time O(num_steps * avg_outdegree), Space O(states). (Exact counts depend on RNG.)
package main

import (
	"fmt"
	"math/rand"
)

type edge struct {
	dst  byte
	prob float64
}

func simulate(start byte, numSteps int, transitions [][]interface{}, seed int64) map[byte]int {
	trans := map[byte][]edge{}
	for _, tr := range transitions {
		src := tr[0].(byte)
		dst := tr[1].(byte)
		prob := tr[2].(float64)
		trans[src] = append(trans[src], edge{dst, prob})
	}
	rng := rand.New(rand.NewSource(seed))
	counts := map[byte]int{}
	state := start
	for i := 0; i < numSteps; i++ {
		counts[state]++
		r := rng.Float64()
		cum := 0.0
		for _, e := range trans[state] {
			cum += e.prob
			if r < cum {
				state = e.dst
				break
			}
		}
	}
	return counts
}

func main() {
	transitions := [][]interface{}{
		{byte('a'), byte('a'), 0.9}, {byte('a'), byte('b'), 0.075}, {byte('a'), byte('c'), 0.025},
		{byte('b'), byte('a'), 0.15}, {byte('b'), byte('b'), 0.8}, {byte('b'), byte('c'), 0.05},
		{byte('c'), byte('a'), 0.25}, {byte('c'), byte('b'), 0.25}, {byte('c'), byte('c'), 0.5},
	}
	c := simulate('a', 5000, transitions, 42)
	fmt.Printf("{'a': %d, 'b': %d, 'c': %d}\n", c['a'], c['b'], c['c'])
}
