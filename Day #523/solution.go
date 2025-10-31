// a+b = (a^b) + 2*(a&b). So c=(M-N)/2 must be a valid carry disjoint from N.
// Ordered pairs = 2^popcount(N), minus the two zero-cases when c==0. O(log M).
package main

import (
	"fmt"
	"math/bits"
)

func countPairs(M, N uint64) int64 {
	if M < N || (M-N)&1 == 1 {
		return 0
	}
	c := (M - N) / 2
	if c&N != 0 {
		return 0
	}
	ways := int64(1) << bits.OnesCount64(N)
	if c == 0 {
		ways -= 2
	}
	if ways < 0 {
		return 0
	}
	return ways
}

func main() {
	fmt.Println(countPairs(10, 4)) // 2
}
