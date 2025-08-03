// Von Neumann extractor: toss biased coin twice; (0,1)->0, (1,0)->1, else retry => 50/50. Time O(1) expected.
package main

import (
	"fmt"
	"math/rand"
)

var rng = rand.New(rand.NewSource(12345))

func tossBiased() int { // simulated bias p = 0.3 for 1
	if rng.Float64() < 0.3 {
		return 1
	}
	return 0
}

func tossFair() int {
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
	trials := 100000
	ones := 0
	for i := 0; i < trials; i++ {
		ones += tossFair()
	}
	frac := float64(ones) / float64(trials)
	if !(frac > 0.48 && frac < 0.52) {
		panic("not fair")
	}
	fmt.Println("Fair coin ~0.5")
}
