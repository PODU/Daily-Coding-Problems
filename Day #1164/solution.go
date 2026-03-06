// E[T] = sum_t (1 - (1-p)^n - n*p*(1-p)^(n-1)), p=2^-t (P(>1 coin alive after t rounds)). Sum until negligible. O(iterations) time, O(1) space.
package main

import (
	"fmt"
	"math"
)

func expectedRounds(n int) float64 {
	total := 0.0
	nf := float64(n)
	for t := 0; t < 1000; t++ {
		p := math.Pow(2.0, float64(-t))
		q := 1.0 - p
		term := 1.0 - math.Pow(q, nf) - nf*p*math.Pow(q, nf-1)
		total += term
		if t > 0 && term < 1e-15 {
			break
		}
	}
	return total
}

func main() {
	n := 4
	fmt.Printf("Expected rounds for n=%d: %.4f\n", n, expectedRounds(n))
}
