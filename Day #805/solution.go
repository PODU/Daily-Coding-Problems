// Day 805: Spreadsheet column number -> alphabetical label (bijective base 26).
// Repeatedly take (n-1)%26 for the letter, then n=(n-1)/26. Time O(log n), Space O(log n).
package main

import "fmt"

func columnLabel(n int) string {
	var s []byte
	for n > 0 {
		n--
		s = append([]byte{byte('A' + n%26)}, s...)
		n /= 26
	}
	return string(s)
}

func main() {
	fmt.Printf("%q\n", columnLabel(1))  // "A"
	fmt.Printf("%q\n", columnLabel(27)) // "AA"
}
