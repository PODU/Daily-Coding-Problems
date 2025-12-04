// Day 691: Bitwise AND of all integers in [M, N].
// Approach: result is the common binary prefix of M and N. Shift both right until
// equal, then shift back. Time O(log N), Space O(1).
package main

import "fmt"

func rangeBitwiseAnd(m, n int) int {
	shift := 0
	for m < n {
		m >>= 1
		n >>= 1
		shift++
	}
	return m << shift
}

func main() {
	fmt.Println(rangeBitwiseAnd(5, 7))   // 4
	fmt.Println(rangeBitwiseAnd(12, 15)) // 12
}
