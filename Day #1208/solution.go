// Day 1208: Fewest perfect squares summing to N.
// Lagrange four-square + Legendre's three-square theorem. Time O(sqrt N), Space O(1).
package main

import (
	"fmt"
	"math"
)

func isSquare(n int) bool {
	r := int(math.Sqrt(float64(n)))
	for r*r < n {
		r++
	}
	for r*r > n {
		r--
	}
	return r*r == n
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
	fmt.Println(numSquares(17)) // 2 (16 + 1)
}
