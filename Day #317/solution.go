// Bitwise AND of all integers in [M, N] = common binary prefix.
// Shift both right until equal, then shift back. Time O(log N), Space O(1).
package main

import "fmt"

func rangeAnd(m, n int) int {
	shift := 0
	for m != n {
		m >>= 1
		n >>= 1
		shift++
	}
	return m << shift
}

func main() {
	M, N := 5, 7
	fmt.Printf("Bitwise AND of [%d, %d] = %d\n", M, N, rangeAnd(M, N))
}
