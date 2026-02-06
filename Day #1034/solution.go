// Day 1034: Expected rounds until one of n fair coins remains.
// Model: expected max of n iid Geometric(1/2) lifetimes; E = sum_{m>=0}(1-(1-2^-m)^n). O(iter).
package main

import (
	"fmt"
	"math"
)

func expectedRounds(n int) float64 {
	e := 0.0
	p := 1.0 // p = 2^-m
	for m := 0; m < 100000; m++ {
		term := 1.0 - math.Pow(1.0-p, float64(n))
		if term < 1e-12 && m > 0 {
			break
		}
		e += term
		p *= 0.5
	}
	return e
}

func main() {
	n := 4
	fmt.Printf("n=%d -> expected rounds: %.4f\n", n, expectedRounds(n))
}
