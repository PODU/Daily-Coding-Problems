// Square root of a real number via Newton's method: x <- (x + n/x)/2.
// Quadratic convergence -> O(log(1/eps)) iterations, O(1) space.
package main

import (
	"fmt"
	"math"
)

func mySqrt(n float64) float64 {
	if n < 0 {
		panic("negative")
	}
	if n == 0 {
		return 0
	}
	x := n
	for i := 0; i < 100; i++ {
		nx := 0.5 * (x + n/x)
		if math.Abs(nx-x) < 1e-15 {
			break
		}
		x = nx
	}
	return x
}

func main() {
	r := mySqrt(9)
	if math.Abs(r-math.Round(r)) < 1e-9 {
		fmt.Println(int64(math.Round(r))) // 3
	} else {
		fmt.Println(r)
	}
}
