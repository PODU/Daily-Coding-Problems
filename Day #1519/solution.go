// Two dice games via Monte Carlo simulation. Expected rolls: "5 then 6"=36, "5 then 5"=42.
// Time: O(trials * rolls_per_trial). Space: O(1).
package main

import (
	"fmt"
	"math/rand"
)

func simulate(first, second int, trials int64, rng *rand.Rand) float64 {
	var total int64
	for t := int64(0); t < trials; t++ {
		prev, rolls := 0, 0
		for {
			r := rng.Intn(6) + 1
			rolls++
			if prev == first && r == second {
				break
			}
			prev = r
		}
		total += int64(rolls)
	}
	return float64(total) / float64(trials)
}

func main() {
	rng := rand.New(rand.NewSource(12345))
	var trials int64 = 500000
	e1 := simulate(5, 6, trials, rng)
	e2 := simulate(5, 5, trials, rng)
	fmt.Printf("Game 1 (five then six) expected rolls: %.2f\n", e1)
	fmt.Printf("Game 2 (five then five) expected rolls: %.2f\n", e2)
	fmt.Println("Alice should play: Game 1 (five then six)")
}
