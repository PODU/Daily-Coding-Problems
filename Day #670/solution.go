// Day 670: Fewest perfect squares summing to n. Lagrange (answer in {1,2,3,4}) +
// Legendre three-square theorem. Time O(sqrt n), Space O(1).
package main

import (
	"fmt"
	"math"
)

func isSquare(n int) bool {
	r := int(math.Sqrt(float64(n)))
	return r*r == n || (r+1)*(r+1) == n
}

func numSquares(n int) int {
	if isSquare(n) {
		return 1
	}
	for a := 1; a*a <= n; a++ {
		if isSquare(n - a*a) {
			return 2
		}
	}
	m := n
	for m%4 == 0 {
		m /= 4
	}
	if m%8 == 7 {
		return 4
	}
	return 3
}

func main() {
	fmt.Println(numSquares(13)) // 2
	fmt.Println(numSquares(27)) // 3
}
