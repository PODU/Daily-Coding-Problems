// Day 109: Swap adjacent bit pairs: ((x&0xAA)>>1)|((x&0x55)<<1). O(1).
package main

import "fmt"

func swapBits(x uint8) uint8 {
	return ((x & 0xAA) >> 1) | ((x & 0x55) << 1)
}

func main() {
	fmt.Printf("%08b\n", swapBits(0b10101010)) // 01010101
	fmt.Printf("%08b\n", swapBits(0b11100010)) // 11010001
}
