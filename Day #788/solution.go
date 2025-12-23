// Integer palindrome check by arithmetic reversal (no string). O(log n) time, O(1) space. Negatives -> false.
package main

import "fmt"

func isPalindrome(n int) bool {
	if n < 0 {
		return false
	}
	rev, x := 0, n
	for x > 0 {
		rev = rev*10 + x%10
		x /= 10
	}
	return rev == n
}

func main() {
	fmt.Println(isPalindrome(121))
}
