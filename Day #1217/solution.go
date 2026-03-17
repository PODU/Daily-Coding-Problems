// Count total set bits in 1..N using per-bit cycle formula. O(log N) time, O(1) space.
package main

import "fmt"

func countSetBits(n int64) int64 {
	var total int64 = 0
	for p := int64(2); p <= 2*n; p <<= 1 {
		full := ((n + 1) / p) * (p / 2)
		rem := (n+1)%p - p/2
		if rem < 0 {
			rem = 0
		}
		total += full + rem
	}
	return total
}

func main() {
	fmt.Println(countSetBits(5))
}
