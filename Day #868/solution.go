// Day 868: Can any permutation of the string form a palindrome?
// Approach: count chars; palindrome possible iff at most one char has an odd count.
// Time: O(n), Space: O(alphabet).
package main

import "fmt"

func canFormPalindrome(s string) bool {
	cnt := map[rune]int{}
	for _, c := range s {
		cnt[c]++
	}
	odd := 0
	for _, v := range cnt {
		odd += v & 1
	}
	return odd <= 1
}

func main() {
	fmt.Println(canFormPalindrome("carrace")) // true
	fmt.Println(canFormPalindrome("daily"))   // false
}
