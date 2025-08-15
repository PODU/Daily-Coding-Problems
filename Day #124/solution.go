// Day 124: Expected flipping rounds until one coin remains.
// DP: E[n](1-2^-n) = 1 + sum_{k<n} P(k survive)*E[k]. O(n^2) time, O(n) space.
package main

import (
	"fmt"
	"math"
)

func expectedRounds(n int) float64 {
	E := make([]float64, n+1)
	for m := 2; m <= n; m++ {
		p := math.Pow(0.5, float64(m)) // p_0
		s := 0.0
		for k := 0; k < m; k++ {
			s += p * E[k]
			p = p * float64(m-k) / float64(k+1)
		}
		pn := math.Pow(0.5, float64(m))
		E[m] = (1.0 + s) / (1.0 - pn)
	}
	if n < 1 {
		return 0.0
	}
	return E[n]
}

func main() {
	for _, n := range []int{1, 2, 3, 4, 5, 10} {
		fmt.Printf("n=%-2d expected rounds = %.6f\n", n, expectedRounds(n))
	}
}
