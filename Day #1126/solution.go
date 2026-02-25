// Smallest number of perfect squares summing to N.
// Legendre/Lagrange: 1 if perfect square; 4 if N=4^a(8b+7); 2 if sum of two squares; else 3. O(sqrt(N)) time, O(1) space.
package main

import (
	"fmt"
	"math"
)

func isSquare(n int64) bool {
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
	for a := int64(1); a*a <= n; a++ {
		if isSquare(n - a*a) {
			return 2
		}
	}
	return 3
}

func main() {
	var N int64 = 17
	fmt.Println(numSquares(N))
}
