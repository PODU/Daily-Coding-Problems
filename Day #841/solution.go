// Day 841: count positive integer pairs (a,b) with a+b=M and a^b=N.
// Math: a+b = (a^b) + 2*(a&b) => a&b=(M-N)/2; answer = 2^popcount(N) minus zero-cases. O(1).
package main

import (
	"fmt"
	"math/bits"
)

func countPairs(M, N int) int {
	d := M - N
	if d < 0 || d&1 != 0 {
		return 0
	}
	c := d / 2 // c = a & b
	if c&N != 0 {
		return 0
	}
	res := 1 << bits.OnesCount(uint(N))
	if c == 0 {
		if N != 0 {
			res -= 2
		} else {
			res -= 1
		}
	}
	if res < 0 {
		return 0
	}
	return res
}

func main() {
	fmt.Println(countPairs(4, 2)) // 2
}
