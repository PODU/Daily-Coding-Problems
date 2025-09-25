// Count ordered (a,b), a+b=M, a^b=N. c=(M-N)/2; valid if c&N==0.
// Count=2^popcount(N), minus 2 if M==N. Time O(1), Space O(1).
package main

import (
	"fmt"
	"math/bits"
)

func countPairs(M, N int64) int64 {
	diff := M - N
	if diff < 0 || diff&1 == 1 {
		return 0
	}
	c := diff / 2
	if c&N != 0 {
		return 0
	}
	count := int64(1) << bits.OnesCount64(uint64(N))
	if M == N {
		count -= 2
	}
	if count < 0 {
		return 0
	}
	return count
}

func main() {
	fmt.Println(countPairs(10, 4))
}
