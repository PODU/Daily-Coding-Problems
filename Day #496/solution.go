// Day 496: Total set bits across 1..N.
// For each bit position, count how many numbers in [0,N] have it set using the
// periodic pattern. O(log N) time, O(1) space.
package main

import "fmt"

func countSetBits(n int64) int64 {
	var total int64 = 0
	for bit := int64(1); bit <= n; bit <<= 1 {
		full := n + 1     // count of integers in [0, n]
		cycle := bit << 1 // period for this bit
		total += (full / cycle) * bit
		rem := full % cycle
		if rem-bit > 0 {
			total += rem - bit
		}
	}
	return total
}

func main() {
	fmt.Println(countSetBits(5))  // 7  (1+1+2+1+2)
	fmt.Println(countSetBits(16)) // 33
}
