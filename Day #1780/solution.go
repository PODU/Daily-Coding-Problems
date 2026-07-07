// Swap even/odd bits of 8-bit int: ((n&0xAA)>>1)|((n&0x55)<<1), masked to 8 bits.
// Time: O(1), Space: O(1).
package main

import (
	"fmt"
	"strconv"
)

func swapBits(bin string) string {
	v, _ := strconv.ParseInt(bin, 2, 32)
	n := int(v)
	r := (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF
	return fmt.Sprintf("%08b", r)
}

func main() {
	fmt.Println(swapBits("10101010"))
	fmt.Println(swapBits("11100010"))
}
