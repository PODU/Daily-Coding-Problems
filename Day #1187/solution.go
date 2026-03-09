// Markov chain simulation: cumulative transition table, draw uniform RNG per step.
// Result is stochastic/approximate (fixed seed for reproducibility). Time O(steps), Space O(states^2).
package main

import (
	"fmt"
	"math/rand"
)

type entry struct {
	cum float64
	to  byte
}

func simulate(start byte, numSteps int, transitions [][3]interface{}, seed int64) map[byte]int {
	raw := map[byte][][2]interface{}{}
	order := []byte{}
	for _, t := range transitions {
		frm := t[0].(byte)
		if _, ok := raw[frm]; !ok {
			order = append(order, frm)
		}
		raw[frm] = append(raw[frm], [2]interface{}{t[1].(byte), t[2].(float64)})
	}
	table := map[byte][]entry{}
	for frm, lst := range raw {
		cum := 0.0
		for _, p := range lst {
			cum += p[1].(float64)
			table[frm] = append(table[frm], entry{cum, p[0].(byte)})
		}
	}

	rng := rand.New(rand.NewSource(seed))
	counts := map[byte]int{'a': 0, 'b': 0, 'c': 0}
	state := start
	for i := 0; i < numSteps; i++ {
		r := rng.Float64()
		for _, e := range table[state] {
			if r < e.cum {
				state = e.to
				break
			}
		}
		counts[state]++
	}
	return counts
}

func main() {
	transitions := [][3]interface{}{
		{byte('a'), byte('a'), 0.9}, {byte('a'), byte('b'), 0.075}, {byte('a'), byte('c'), 0.025},
		{byte('b'), byte('a'), 0.15}, {byte('b'), byte('b'), 0.8}, {byte('b'), byte('c'), 0.05},
		{byte('c'), byte('a'), 0.25}, {byte('c'), byte('b'), 0.25}, {byte('c'), byte('c'), 0.5},
	}
	counts := simulate('a', 5000, transitions, 12345)
	fmt.Printf("{'a': %d, 'b': %d, 'c': %d}\n", counts['a'], counts['b'], counts['c'])
}
