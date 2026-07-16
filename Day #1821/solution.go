// Square root via Newton's method. ~O(log(1/eps)) iterations, O(1) space.
package main

import (
	"fmt"
	"math"
)

func mysqrt(n float64) float64 {
	if n == 0 {
		return 0
	}
	x := n
	for i := 0; i < 200; i++ {
		nx := 0.5 * (x + n/x)
		if math.Abs(nx-x) < 1e-15 {
			x = nx
			break
		}
		x = nx
	}
	return x
}

func main() {
	n := 9.0
	r := mysqrt(n)
	if math.Abs(r-math.Round(r)) < 1e-9 {
		fmt.Println(int64(math.Round(r)))
	} else {
		fmt.Println(r)
	}
}
