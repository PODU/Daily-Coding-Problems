// Palindrome integer check by reversing the number arithmetically (no string). Negatives are not palindromes. O(log10(n)).
package main

import "fmt"

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}
	original, reversed := x, 0
	for x > 0 {
		reversed = reversed*10 + x%10
		x /= 10
	}
	return reversed == original
}

func main() {
	fmt.Println(isPalindrome(121)) // true
}
