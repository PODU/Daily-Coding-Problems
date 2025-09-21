// Day 310: Total set bits over 1..N. Per-bit counting. O(log N).
package main

import "fmt"

func countBits(N int64) int64 {
	var total int64
	for i := int64(0); (int64(1) << uint(i)) <= N; i++ {
		blk := int64(1) << uint(i+1)
		full := (N + 1) / blk * (int64(1) << uint(i))
		rem := (N+1)%blk - (int64(1) << uint(i))
		if rem < 0 {
			rem = 0
		}
		total += full + rem
	}
	return total
}

func main() {
	fmt.Println(countBits(5)) // 7
}
