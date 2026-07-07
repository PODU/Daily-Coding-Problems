// Min squared integers summing to n via Lagrange four-square + Legendre three-square theorem.
// O(sqrt(n)) per query (only the i^2+j^2 check loops), O(1) space.
package main

import (
	"fmt"
	"math"
)

func isPerfectSquare(n int64) bool {
	r := int64(math.Sqrt(float64(n)))
	for r*r < n {
		r++
	}
	for r*r > n {
		r--
	}
	return r*r == n
}

func numSquares(n int64) int {
	if isPerfectSquare(n) {
		return 1
	}
	m := n
	for m%4 == 0 {
		m /= 4
	}
	if m%8 == 7 {
		return 4
	}
	for i := int64(1); i*i <= n; i++ {
		if isPerfectSquare(n - i*i) {
			return 2
		}
	}
	return 3
}

func main() {
	fmt.Println(numSquares(13)) // 2
	fmt.Println(numSquares(27)) // 3
}
