// Day 769: Expected rolls for "5 then 6" vs "5 then 5" stopping games.
// Exact via 2-state Markov chains (E1=36, E2=42) plus Monte-Carlo check.
package main

import (
	"fmt"
	"math/rand"
)

func simulate(second int, trials int, rng *rand.Rand) float64 {
	total := 0
	for t := 0; t < trials; t++ {
		prev, rolls := 0, 0
		for {
			r := rng.Intn(6) + 1
			rolls++
			if prev == 5 && r == second {
				break
			}
			prev = r
		}
		total += rolls
	}
	return float64(total) / float64(trials)
}

func main() {
	rng := rand.New(rand.NewSource(12345))
	trials := 300000
	fmt.Printf("Game 1 (5 then 6): exact=36, simulated=%.2f\n", simulate(6, trials, rng))
	fmt.Printf("Game 2 (5 then 5): exact=42, simulated=%.2f\n", simulate(5, trials, rng))
	fmt.Println("Alice should play Game 1 (5 then 6); it has the lower expected cost.")
}
