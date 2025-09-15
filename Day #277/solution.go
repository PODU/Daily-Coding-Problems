// Day 277: Validate UTF-8 from byte-value array. Single pass.
// Time O(N), Space O(1). Only low 8 bits of each integer are used.
package main

import "fmt"

func validUTF8(data []int) bool {
	remaining := 0
	for _, v := range data {
		b := v & 0xFF
		if remaining == 0 {
			switch {
			case b>>7 == 0:
				remaining = 0
			case b>>5 == 0b110:
				remaining = 1
			case b>>4 == 0b1110:
				remaining = 2
			case b>>3 == 0b11110:
				remaining = 3
			default:
				return false
			}
		} else {
			if b>>6 != 0b10 {
				return false
			}
			remaining--
		}
	}
	return remaining == 0
}

func main() {
	fmt.Println(validUTF8([]int{0b11100010, 0b10000010, 0b10101100})) // true (Euro sign)
	fmt.Println(validUTF8([]int{0b11101011, 0b10001100, 0b00000100})) // false
}
