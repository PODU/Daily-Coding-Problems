// Square root via Newton's method: x = (x + n/x)/2, quadratic convergence.
// Time: O(log(1/eps)) iterations, Space: O(1).
package main

import (
	"fmt"
	"math"
)

func mySqrt(n float64) float64 {
	if n < 0 {
		return math.NaN()
	}
	if n == 0 {
		return 0
	}
	x := n
	for i := 0; i < 100; i++ {
		nx := 0.5 * (x + n/x)
		if math.Abs(nx-x) < 1e-15 {
			x = nx
			break
		}
		x = nx
	}
	return x
}

func printResult(n float64) {
	r := mySqrt(n)
	ri := math.Round(r)
	if math.Abs(r-ri) < 1e-9 {
		fmt.Printf("%d\n", int64(ri)) // exact integer
	} else {
		fmt.Printf("%.8f\n", r)
	}
}

func main() {
	printResult(9) // -> 3
	printResult(2) // -> 1.41421356
}
