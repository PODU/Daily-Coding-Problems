// Day 719: Convert 1-based column number to spreadsheet id (bijective base-26).
// Repeatedly take (n-1)%26 then n=(n-1)/26. Time O(log n).
package main

import "fmt"

func colId(n int) string {
	b := []byte{}
	for n > 0 {
		n--
		b = append([]byte{byte('A' + n%26)}, b...)
		n /= 26
	}
	return string(b)
}

func main() {
	fmt.Printf("\"%s\"\n", colId(1))
	fmt.Printf("\"%s\"\n", colId(27))
}
