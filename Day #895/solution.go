// Palindrome by deleting at most k chars: min deletions = n - LPS(s).
// LPS via interval DP (space-optimized to O(n)). Time O(n^2), Space O(n).
package main

import "fmt"

func canMakePalindrome(s string, k int) bool {
	n := len(s)
	if n == 0 {
		return 0 <= k
	}
	prev := make([]int, n)
	cur := make([]int, n)
	for i := n - 1; i >= 0; i-- {
		cur = make([]int, n)
		cur[i] = 1
		for j := i + 1; j < n; j++ {
			if s[i] == s[j] {
				cur[j] = prev[j-1] + 2
			} else if prev[j] > cur[j-1] {
				cur[j] = prev[j]
			} else {
				cur[j] = cur[j-1]
			}
		}
		prev = cur
	}
	lps := cur[n-1]
	return (n - lps) <= k
}

func main() {
	if canMakePalindrome("waterrfetawx", 2) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
