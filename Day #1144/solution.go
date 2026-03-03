// Day 1144: count positive pairs (a,b) with a+b=M, a^b=N.
// a+b = (a^b) + 2*(a&b) => and=(M-N)/2; valid iff M>=N, even, and&N==0.
// Ordered pairs = 2^popcount(N), minus 2 if and==0 (excludes a=0/b=0). O(1).
package main

import (
	"fmt"
	"math/bits"
)

func countPairs(M, N int64) int64 {
	if M < N || (M-N)&1 == 1 {
		return 0
	}
	aAnd := (M - N) / 2
	if aAnd&N != 0 {
		return 0
	}
	cnt := int64(1) << bits.OnesCount64(uint64(N))
	if aAnd == 0 {
		cnt -= 2
	}
	if cnt < 0 {
		return 0
	}
	return cnt
}

func main() {
	fmt.Println(countPairs(10, 4)) // 2 -> (7,3) and (3,7)
}
