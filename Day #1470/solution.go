// Least squares summing to n via Legendre/Lagrange: 1 if square, 4 if 4^a(8b+7),
// 2 if i^2+j^2, else 3. Time: O(sqrt(n)); Space: O(1).
package main

import (
	"fmt"
	"math"
)

func isSquare(x int64) bool {
	r := int64(math.Sqrt(float64(x)))
	for r*r < x {
		r++
	}
	for r*r > x {
		r--
	}
	return r*r == x
}

func numSquares(n int64) int {
	if isSquare(n) {
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
		if isSquare(n - i*i) {
			return 2
		}
	}
	return 3
}

func main() {
	fmt.Println(numSquares(13))
	fmt.Println(numSquares(27))
}
