// Day 716: Validate UTF-8. Read leading byte to get char length (1-4) from its
// high bits, then verify each continuation byte starts with 10. Time O(n).
package main

import "fmt"

func validUtf8(data []int) bool {
	i, n := 0, len(data)
	for i < n {
		b := data[i] & 0xFF
		var length int
		switch {
		case b>>7 == 0b0:
			length = 1
		case b>>5 == 0b110:
			length = 2
		case b>>4 == 0b1110:
			length = 3
		case b>>3 == 0b11110:
			length = 4
		default:
			return false
		}
		if i+length > n {
			return false
		}
		for j := 1; j < length; j++ {
			if (data[i+j]&0xFF)>>6 != 0b10 {
				return false
			}
		}
		i += length
	}
	return true
}

func b2s(b bool) string {
	if b {
		return "True"
	}
	return "False"
}

func main() {
	fmt.Println(b2s(validUtf8([]int{226, 130, 172})))
	fmt.Println(b2s(validUtf8([]int{235, 140, 4})))
}
