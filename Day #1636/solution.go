// Day 1636: Can a string be made a palindrome by deleting at most k chars.
// min deletions = n - LongestPalindromicSubsequence; DP O(n^2) time/space.
package main

import "fmt"

func canMakePalindrome(s string, k int) bool {
	n := len(s)
	if n == 0 {
		return true
	}
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
		dp[i][i] = 1
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			if s[i] == s[j] {
				inner := 0
				if length > 2 {
					inner = dp[i+1][j-1]
				}
				dp[i][j] = 2 + inner
			} else {
				if dp[i+1][j] > dp[i][j-1] {
					dp[i][j] = dp[i+1][j]
				} else {
					dp[i][j] = dp[i][j-1]
				}
			}
		}
	}
	lps := dp[0][n-1]
	return (n - lps) <= k
}

func main() {
	if canMakePalindrome("waterrfetawx", 2) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
