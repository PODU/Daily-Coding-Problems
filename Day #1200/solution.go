// Reverse all 32 bits by shifting LSB of input into LSB-first of result.
// Time: O(1) (32 steps); Space: O(1).
package main

import (
	"fmt"
	"strings"
)

func reverseBits(x uint32) uint32 {
	var r uint32 = 0
	for i := 0; i < 32; i++ {
		r = (r << 1) | (x & 1)
		x >>= 1
	}
	return r
}

func toGrouped(x uint32) string {
	var b strings.Builder
	for i := 31; i >= 0; i-- {
		if (x>>uint(i))&1 == 1 {
			b.WriteByte('1')
		} else {
			b.WriteByte('0')
		}
		if i%4 == 0 && i != 0 {
			b.WriteByte(' ')
		}
	}
	return b.String()
}

func main() {
	var input uint32 = 0xF0F0F0F0
	fmt.Println("Input: ", toGrouped(input))
	fmt.Println(toGrouped(reverseBits(input)))
}
