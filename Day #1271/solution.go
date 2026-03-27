// Day 1271: Implement rand5() from rand7() with uniform probability.
// Rejection sampling: redraw rand7 until result <= 5. Expected O(7/5) calls per sample.
package main

import (
	"fmt"
	"math/rand"
)

func rand7() int { return rand.Intn(7) + 1 }

func rand5() int {
	v := rand7()
	for v > 5 { // reject 6,7 -> uniform 1..5
		v = rand7()
	}
	return v
}

func main() {
	trials := 100000
	count := make([]int, 6)
	for i := 0; i < trials; i++ {
		count[rand5()]++
	}
	fmt.Printf("Distribution over %d samples (expect ~%d each):\n", trials, trials/5)
	for v := 1; v <= 5; v++ {
		fmt.Printf("%d: %d\n", v, count[v])
	}
}
