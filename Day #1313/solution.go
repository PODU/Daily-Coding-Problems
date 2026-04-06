// Bitwise AND of all integers in [M, N] = common binary prefix of M and N.
// Shift both right until equal, then shift back. Time O(log N), Space O(1).
package main

import "fmt"

func rangeAnd(m, n int64) int64 {
	shift := 0
	for m < n {
		m >>= 1
		n >>= 1
		shift++
	}
	return m << shift
}

func main() {
	fmt.Println(rangeAnd(5, 7)) // 4  (5 & 6 & 7)
}
