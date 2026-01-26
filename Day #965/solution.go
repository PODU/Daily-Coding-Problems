// Day 965: Validate whether byte values form a valid UTF-8 encoding.
// Approach: count leading 1s of lead byte, verify continuation bytes. Time O(n), Space O(1).
package main

import "fmt"

func validUtf8(data []int) bool {
	remaining := 0
	for _, b := range data {
		b &= 0xFF
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
	fmt.Println(validUtf8([]int{0b11100010, 0b10000010, 0b10101100})) // true (Euro sign)
	fmt.Println(validUtf8([]int{0b11110000, 0b10000000}))             // false (truncated)
}
