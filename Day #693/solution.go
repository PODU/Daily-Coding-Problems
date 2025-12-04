// Day 693: Swap adjacent (even/odd) bits of an unsigned 8-bit integer.
// Approach: one-liner masks. Odd bits 0x55 shift left, even bits 0xAA shift right.
// Time O(1), Space O(1).
package main

import "fmt"

func swapBits(n uint8) uint8 {
	return ((n & 0xAA) >> 1) | ((n & 0x55) << 1)
}

func main() {
	fmt.Printf("%08b\n", swapBits(0b10101010)) // 01010101
	fmt.Printf("%08b\n", swapBits(0b11100010)) // 11010001
}
