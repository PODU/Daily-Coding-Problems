// Max of two ints without if/else/branch/ternary/comparison.
// Use sign bit of (a-b) via 64-bit diff to avoid overflow. O(1) time, O(1) space.
package main

import "fmt"

func maxOf(a, b int64) int64 {
	d := a - b               // safe in 64-bit for 32-bit inputs
	sign := (d >> 63) & 1    // 1 if a<b, else 0
	return a - sign*d        // a - (a-b) = b when a<b, else a
}

func main() {
	fmt.Printf("max(3, 7) = %d\n", maxOf(3, 7))
	fmt.Printf("max(10, 2) = %d\n", maxOf(10, 2))
}
