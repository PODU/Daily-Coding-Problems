// Palindrome permutation: count char parities; valid iff <=1 char has odd count. O(n) time, O(1) space.
package main

import "fmt"

func canPermutePalindrome(s string) bool {
	cnt := make(map[rune]int)
	for _, c := range s {
		cnt[c]++
	}
	odd := 0
	for _, v := range cnt {
		if v&1 == 1 {
			odd++
		}
	}
	return odd <= 1
}

func main() {
	fmt.Println(canPermutePalindrome("carrace"))
	fmt.Println(canPermutePalindrome("daily"))
}
