// Day 1303: Next larger integer with the same number of set bits (snoob).
// Bit-hack: add lowest set bit, then re-insert the moved bits at the bottom. O(1) time.
package main

import "fmt"

func nextSameBits(n uint64) uint64 {
	c := n & (-n)             // lowest set bit
	r := n + c                // ripple carry
	ones := ((n ^ r) >> 2) / c // moved bits, shifted down
	return r | ones
}

func main() {
	fmt.Println(nextSameBits(6)) // 9
}
