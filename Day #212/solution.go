// Day 212: Spreadsheet column number -> label (bijective base-26).
// Approach: repeatedly take (n-1)%26 for the digit, then n=(n-1)/26. Time O(log n), Space O(log n).
package main

import "fmt"

func columnLabel(n int) string {
	b := []byte{}
	for n > 0 {
		n--
		b = append(b, byte('A'+n%26))
		n /= 26
	}
	for i, j := 0, len(b)-1; i < j; i, j = i+1, j-1 {
		b[i], b[j] = b[j], b[i]
	}
	return string(b)
}

func main() {
	fmt.Printf("\"%s\"\n", columnLabel(1))
	fmt.Printf("\"%s\"\n", columnLabel(27))
}
