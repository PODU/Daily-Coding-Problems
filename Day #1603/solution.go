// Total set bits from 1..N. Per-bit counting: O(log N) time, O(1) space.
// For each bit i: full cycles contribute 2^i ones each, plus remainder.
package main

import "fmt"

func countSetBits(n int64) int64 {
	var total int64 = 0
	for i := int64(0); (int64(1) << i) <= n; i++ {
		block := int64(1) << (i + 1)
		ones := (n + 1) / block * (int64(1) << i)
		rem := (n+1)%block - (int64(1) << i)
		if rem > 0 {
			ones += rem
		}
		total += ones
	}
	return total
}

func main() {
	fmt.Println("N=5  ->", countSetBits(5))   // 7
	fmt.Println("N=16 ->", countSetBits(16))  // 33
}
