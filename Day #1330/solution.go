// Day 1330: Column number -> spreadsheet label (bijective base-26).
// Repeatedly take (n-1)%26 for the next letter, then n=(n-1)/26. O(log n) time.
package main

import "fmt"

func columnTitle(n int) string {
	b := []byte{}
	for n > 0 {
		n--
		b = append([]byte{byte('A' + n%26)}, b...)
		n /= 26
	}
	return string(b)
}

func main() {
	fmt.Printf("%q\n", columnTitle(1))  // "A"
	fmt.Printf("%q\n", columnTitle(27)) // "AA"
}
