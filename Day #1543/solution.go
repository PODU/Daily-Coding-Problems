// Estimate pi via Monte Carlo: fraction of random points in [0,1]^2 inside the
// unit quarter-circle approximates pi/4. Time O(samples), Space O(1).
package main

import (
	"fmt"
	"math/rand"
)

func estimatePi(samples int64, seed int64) float64 {
	rng := rand.New(rand.NewSource(seed))
	var inside int64
	for i := int64(0); i < samples; i++ {
		x, y := rng.Float64(), rng.Float64()
		if x*x+y*y <= 1.0 {
			inside++
		}
	}
	return 4.0 * float64(inside) / float64(samples)
}

func main() {
	pi := estimatePi(10000000, 42)
	fmt.Printf("%.3f\n", pi)
}
