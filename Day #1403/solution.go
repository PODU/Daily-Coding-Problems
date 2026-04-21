// Monte-Carlo simulation of two stop conditions plus exact theory.
// E[rolls until 5->6] = 36 (distinct faces); E[rolls until 5->5] = 42 (same face).
// Time O(trials * rolls), Space O(1).
package main

import (
	"fmt"
	"math/rand"
)

func play(second int, rng *rand.Rand) int64 {
	var rolls int64
	prev := 0
	for {
		cur := rng.Intn(6) + 1
		rolls++
		if prev == 5 && cur == second {
			return rolls
		}
		prev = cur
	}
}

func main() {
	rng := rand.New(rand.NewSource(42))
	var trials int64 = 200000
	var s56, s55 int64
	for i := int64(0); i < trials; i++ {
		s56 += play(6, rng)
	}
	for i := int64(0); i < trials; i++ {
		s55 += play(5, rng)
	}
	fmt.Printf("Game 1 (five then six): simulated avg = %.2f, theoretical = 36\n",
		float64(s56)/float64(trials))
	fmt.Printf("Game 2 (five then five): simulated avg = %.2f, theoretical = 42\n",
		float64(s55)/float64(trials))
	fmt.Println("Alice should play Game 1 (five-then-six): fewer expected rolls, less pay.")
}
