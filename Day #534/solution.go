// Permutation-palindrome check: a permutation can be a palindrome iff at most one
// character has an odd frequency. Toggle membership in a set as we scan.
// Time: O(n); Space: O(alphabet).
package main

import "fmt"

func canPermutePalindrome(s string) bool {
	odd := map[rune]bool{}
	for _, ch := range s {
		if odd[ch] {
			delete(odd, ch)
		} else {
			odd[ch] = true
		}
	}
	return len(odd) <= 1
}

func main() {
	fmt.Println(canPermutePalindrome("carrace"))
	fmt.Println(canPermutePalindrome("daily"))
}
