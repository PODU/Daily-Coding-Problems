// Bitwise AND of range [M,N] = common binary prefix; shift both right until equal, then back. O(log N) time, O(1) space.
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
	fmt.Println(rangeBitwiseAnd(5, 7))
}
