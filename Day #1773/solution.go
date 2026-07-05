// Day 1773: Count ordered positive pairs (a,b) with a+b=M, a^b=N.
// Use a+b=(a^b)+2*(a&b): s=(M-N)/2 must satisfy (s&N)==0; answer=2^popcount(N)
// minus the all-or-nothing assignments when s==0. O(1) time, O(1) space.
package main

import (
	"fmt"
	"math/bits"
)

func countPairs(M, N int) int {
	if M-N < 0 || (M-N)&1 == 1 {
		return 0
	}
	s := (M - N) / 2 // s == a & b
	if s&N != 0 {    // carry bits disjoint from xor bits
		return 0
	}
	if N == 0 {
		if M > 0 && M%2 == 0 {
			return 1 // only (M/2, M/2)
		}
		return 0
	}
	ways := 1 << bits.OnesCount(uint(N))
	if s == 0 {
		ways -= 2 // drop a=0 and b=0 to keep both positive
	}
	return ways
}

func main() {
	fmt.Println(countPairs(6, 4)) // -> 2
}
