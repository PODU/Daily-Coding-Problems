// Reverse the 32 bits of a 32-bit integer by shifting result left and pulling
// the lowest bit of the input each iteration. Time O(1) (32 iters), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func reverseBits(n uint32) uint32 {
	var result uint32
	for i := 0; i < 32; i++ {
		result = (result << 1) | (n & 1)
		n >>= 1
	}
	return result
}

func groupNibbles(n uint32) string {
	bits := fmt.Sprintf("%032b", n)
	var parts []string
	for i := 0; i < 32; i += 4 {
		parts = append(parts, bits[i:i+4])
	}
	return strings.Join(parts, " ")
}

func main() {
	var value uint32 = 0xF0F0F0F0
	fmt.Println(groupNibbles(reverseBits(value)))
}
