// Day 157: A permutation is a palindrome iff at most one char has odd count.
// Track parity via a set of odd-count chars. Time O(n), Space O(alphabet).
package main

import "fmt"

func canFormPalindrome(s string) bool {
	odd := make(map[rune]bool)
	for _, c := range s {
		if odd[c] {
			delete(odd, c)
		} else {
			odd[c] = true
		}
	}
	return len(odd) <= 1
}

func main() {
	fmt.Println(canFormPalindrome("carrace")) // true
	fmt.Println(canFormPalindrome("daily"))   // false
}
