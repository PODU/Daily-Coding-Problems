// Day 1021: Swap even and odd bits of an 8-bit integer.
// Approach: ((n & 0xAA) >> 1) | ((n & 0x55) << 1).  Time O(1), Space O(1).
package main

import (
	"fmt"
	"strconv"
)

func swapBits(n uint) uint { return (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF }

func main() {
	for _, b := range []string{"10101010", "11100010"} {
		v, _ := strconv.ParseUint(b, 2, 32)
		fmt.Printf("%08b\n", swapBits(uint(v)))
	}
}
