// Day 129: Square root of a real number via Newton's method.
// Quadratic convergence: O(log(1/eps)) iterations.
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
		if math.Abs(nx-x) < 1e-12 {
			break
		}
		x = nx
	}
	return x
}

func main() {
	n := 9.0
	r := mySqrt(n)
	if math.Abs(r-math.Round(r)) < 1e-9 {
		fmt.Println(int64(math.Round(r)))
	} else {
		fmt.Println(r)
	}
}
