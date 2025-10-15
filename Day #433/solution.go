// Day 433: Next larger integer with the same number of set bits (Gosper's hack).
// c = n & -n; r = n + c; next = (((r ^ n) >> 2) / c) | r. O(1) time, O(1) space.
package main

import (
	"fmt"
	"strconv"
)

func nextSameBits(n int) int {
	if n <= 0 {
		return n
	}
	c := n & (-n)
	r := n + c
	return (((r ^ n) >> 2) / c) | r
}

func main() {
	n := 6
	m := nextSameBits(n)
	fmt.Printf("Input: %d (%s in binary)\n", n, strconv.FormatInt(int64(n), 2))
	fmt.Printf("Next: %d (%s in binary)\n", m, strconv.FormatInt(int64(m), 2))
}
