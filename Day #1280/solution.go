// Day 1280: Swap adjacent bit pairs of an 8-bit unsigned integer.
// One-liner: shift odd bits up, even bits down. Time O(1), Space O(1).
package main

import (
	"fmt"
	"strconv"
)

func swapBits(n uint8) uint8 {
	return ((n & 0xAA) >> 1) | ((n & 0x55) << 1)
}

func main() {
	for _, s := range []string{"10101010", "11100010"} {
		v, _ := strconv.ParseUint(s, 2, 8)
		fmt.Printf("%08b\n", swapBits(uint8(v)))
	}
}
