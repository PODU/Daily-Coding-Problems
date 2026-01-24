// Day 947: smallest sparse number (no two adjacent set bits) >= N.
// Repeatedly fix the lowest adjacent-1 pair by rounding up. Time O(log N), Space O(1).
package main

import (
	"fmt"
	"math/bits"
)

func smallestSparse(n uint64) uint64 {
	for (n & (n >> 1)) != 0 {
		m := n & (n >> 1)
		p := bits.TrailingZeros64(m)
		step := uint64(1) << (p + 1)
		n = (n + step) & ^(step - 1)
	}
	return n
}

func main() {
	fmt.Println(smallestSparse(21)) // 21
	fmt.Println(smallestSparse(22)) // 32
}
