// Day 161: Reverse the 32 bits of an integer by shifting bits out of the input
// into the result. Time O(32), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func reverseBits(n uint32) uint32 {
	var res uint32
	for i := 0; i < 32; i++ {
		res = (res << 1) | (n & 1)
		n >>= 1
	}
	return res
}

func toGroups(n uint32) string {
	var sb strings.Builder
	for i := 31; i >= 0; i-- {
		if (n>>uint(i))&1 == 1 {
			sb.WriteByte('1')
		} else {
			sb.WriteByte('0')
		}
		if i%4 == 0 && i != 0 {
			sb.WriteByte(' ')
		}
	}
	return sb.String()
}

func main() {
	n := uint32(0xF0F0F0F0) // 1111 0000 ... repeated
	fmt.Println(toGroups(reverseBits(n)))
}
