// Next bigger integer with the same number of set bits (Gosper's hack). O(1).
package main

import "fmt"

func nextSamePopcount(n uint) uint {
	c := n & (-n)              // lowest set bit
	r := n + c                 // ripple the carry
	return r | (((n ^ r) >> 2) / c) // restore trailing ones
}

func main() {
	fmt.Println(nextSamePopcount(6)) // 6 (0110) -> 9 (1001)
}
