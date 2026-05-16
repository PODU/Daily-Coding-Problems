// UTF-8 validation: read lead byte, count leading ones (1->1byte, 2..4 multi), verify continuation bytes start with 10.
// Time O(n), Space O(1).
package main

import "fmt"

func validUtf8(data []int) bool {
	i, n := 0, len(data)
	for i < n {
		b := data[i] & 0xFF
		var cnt int
		switch {
		case b&0x80 == 0x00:
			cnt = 1
		case b&0xE0 == 0xC0:
			cnt = 2
		case b&0xF0 == 0xE0:
			cnt = 3
		case b&0xF8 == 0xF0:
			cnt = 4
		default:
			return false
		}
		if i+cnt > n {
			return false
		}
		for k := 1; k < cnt; k++ {
			if data[i+k]&0xC0 != 0x80 {
				return false
			}
		}
		i += cnt
	}
	return true
}

func main() {
	fmt.Println(validUtf8([]int{226, 130, 172}))                  // true
	fmt.Println(validUtf8([]int{0b11110101, 0b10000010, 0b00000010})) // false
}
