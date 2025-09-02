// Day 202: Integer palindrome check without string conversion.
// Reverse the second half of the digits and compare with the first half.
// Time: O(log10 n), Space: O(1).
package main

import "fmt"

func isPalindrome(x int64) bool {
	if x < 0 {
		return false
	}
	if x != 0 && x%10 == 0 {
		return false // trailing zero, not 0 itself
	}
	rev := int64(0)
	for x > rev {
		rev = rev*10 + x%10
		x /= 10
	}
	return x == rev || x == rev/10
}

func main() {
	fmt.Println(isPalindrome(121)) // true
	fmt.Println(isPalindrome(888)) // true
	fmt.Println(isPalindrome(678)) // false
}
