// Estimate Pi via Monte Carlo: sample random points in unit square, fraction inside
// quarter circle ~ pi/4. O(S) for S samples. Fixed seed for reproducible 3-decimal result.
package main

import (
	"fmt"
	"math/rand"
)

func estimatePi(samples int) float64 {
	rng := rand.New(rand.NewSource(12345))
	inside := 0
	for i := 0; i < samples; i++ {
		x, y := rng.Float64(), rng.Float64()
		if x*x+y*y <= 1.0 {
			inside++
		}
	}
	return 4.0 * float64(inside) / float64(samples)
}

func main() {
	fmt.Printf("%.3f\n", estimatePi(20000000)) // ~3.142
}
