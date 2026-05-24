// Palindrome permutation: toggle chars in a set; a permutation is a palindrome
// iff at most one char has an odd count (set size <= 1). Time O(n), Space O(alphabet).
package main

import "fmt"

func canFormPalindrome(s string) bool {
	odd := map[rune]bool{}
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
	s := "carrace"
	if canFormPalindrome(s) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
