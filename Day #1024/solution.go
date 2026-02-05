// Day 1024: Reverse all 32 bits of a 32-bit integer.
// Approach: swap-mask reversal in O(log 32) = O(1) time, O(1) space.
package main

import (
	"fmt"
	"strings"
)

func reverseBits(n uint32) uint32 {
	n = (n >> 16) | (n << 16)
	n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8)
	n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4)
	n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2)
	n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1)
	return n
}

func grouped(n uint32) string {
	bits := fmt.Sprintf("%032b", n)
	var parts []string
	for i := 0; i < 32; i += 4 {
		parts = append(parts, bits[i:i+4])
	}
	return strings.Join(parts, " ")
}

func main() {
	var x uint32 = 0xF0F0F0F0
	fmt.Println(grouped(reverseBits(x)))
}
