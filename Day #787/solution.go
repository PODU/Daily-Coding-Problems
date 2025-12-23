// Next bigger integer with same popcount via bit-hack (Gosper's hack). O(1) time, O(1) space.
package main

import "fmt"

func nextSamePopcount(n uint) uint {
	if n == 0 {
		return 0
	}
	c := n & (-n)            // lowest set bit
	r := n + c               // ripple carry
	ones := ((n ^ r) >> 2) / c // shifted-in ones
	return r | ones
}

func main() {
	fmt.Println(nextSamePopcount(6)) // 0110 -> 1001 = 9
}
