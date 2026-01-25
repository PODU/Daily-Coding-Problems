// Day 959: total number of set bits over all integers in [1, N].
// Per-bit counting of full cycles plus remainder. Time O(log N), Space O(1).
package main

import "fmt"

func countSetBits(n int64) int64 {
	var total int64 = 0
	for i := 0; (int64(1) << i) <= n; i++ {
		cycle := int64(1) << (i + 1)
		half := cycle >> 1
		total += (n + 1) / cycle * half
		rem := (n + 1) % cycle
		if rem-half > 0 {
			total += rem - half
		}
	}
	return total
}

func main() {
	fmt.Println(countSetBits(5)) // 7
}
