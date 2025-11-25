// Reverse 32 bits: iterate 32 times, shift result left, OR in LSB of input, shift input right.
// Time O(32)=O(1), space O(1). Uses uint32 for unsigned 32-bit arithmetic.
package main

import (
	"fmt"
	"strings"
)

func reverseBits(x uint32) uint32 {
	var result uint32 = 0
	for i := 0; i < 32; i++ {
		result = (result << 1) | (x & 1)
		x >>= 1
	}
	return result
}

func main() {
	out := reverseBits(0xF0F0F0F0)
	bits := fmt.Sprintf("%032b", out)
	var groups []string
	for i := 0; i < 32; i += 4 {
		groups = append(groups, bits[i:i+4])
	}
	fmt.Println(strings.Join(groups, " "))
}
