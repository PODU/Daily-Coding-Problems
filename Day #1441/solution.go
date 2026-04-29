// Day 1441: Validate UTF-8 encoding from an array of byte values.
// Approach: scan bytes, read leading-one count of lead byte, verify
// continuation bytes start with 10. Time O(n), Space O(1).
package main

import "fmt"

func validUtf8(data []int) bool {
	i, n := 0, len(data)
	for i < n {
		b := data[i] & 0xFF
		var cnt int
		switch {
		case b>>7 == 0b0:
			cnt = 1
		case b>>5 == 0b110:
			cnt = 2
		case b>>4 == 0b1110:
			cnt = 3
		case b>>3 == 0b11110:
			cnt = 4
		default:
			return false
		}
		if i+cnt > n {
			return false
		}
		for j := 1; j < cnt; j++ {
			if (data[i+j]&0xFF)>>6 != 0b10 {
				return false
			}
		}
		i += cnt
	}
	return true
}

func main() {
	euro := []int{226, 130, 172} // 11100010 10000010 10101100
	if validUtf8(euro) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
