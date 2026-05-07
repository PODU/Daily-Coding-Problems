// Day 1482: Integer palindrome without converting to a string.
// Reverse the number arithmetically and compare. Time O(digits), Space O(1).
package main

import "fmt"

func isPalindrome(x int64) bool {
	if x < 0 {
		return false
	}
	rev, n := int64(0), x
	for n > 0 {
		rev = rev*10 + n%10
		n /= 10
	}
	return rev == x
}

func main() {
	for _, v := range []int64{121, 888, 678} {
		label := "not a palindrome"
		if isPalindrome(v) {
			label = "palindrome"
		}
		fmt.Printf("%d -> %s\n", v, label)
	}
}
