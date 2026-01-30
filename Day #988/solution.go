// Day 988: Minimum number of perfect squares summing to n.
// Number theory (Lagrange + Legendre's three-square theorem). O(sqrt n) time, O(1) space.
package main

import (
	"fmt"
	"math"
)

func isSquare(x int64) bool {
	r := int64(math.Sqrt(float64(x)))
	for r*r > x {
		r--
	}
	for (r+1)*(r+1) <= x {
		r++
	}
	return r*r == x
}

func numSquares(n int64) int {
	if isSquare(n) {
		return 1
	}
	m := n
	for m%4 == 0 { // strip factors of 4
		m /= 4
	}
	if m%8 == 7 { // Legendre: 4^a(8b+7) needs 4
		return 4
	}
	for a := int64(1); a*a <= n; a++ {
		if isSquare(n - a*a) {
			return 2
		}
	}
	return 3
}

func main() {
	fmt.Println("13 ->", numSquares(13)) // expected 2
	fmt.Println("27 ->", numSquares(27)) // expected 3
}
