// Day 411: Can we make s a palindrome by deleting at most k chars?
// Answer: n - LPS(s) <= k, where LPS = longest palindromic subsequence via DP. O(n^2) time, O(n^2) space.
package main

import "fmt"

func lps(s string) int {
	n := len(s)
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
	}
	for i := n - 1; i >= 0; i-- {
		dp[i][i] = 1
		for j := i + 1; j < n; j++ {
			if s[i] == s[j] {
				dp[i][j] = dp[i+1][j-1] + 2
			} else if dp[i+1][j] > dp[i][j-1] {
				dp[i][j] = dp[i+1][j]
			} else {
				dp[i][j] = dp[i][j-1]
			}
		}
	}
	return dp[0][n-1]
}

func canMakePalindrome(s string, k int) bool {
	return len(s)-lps(s) <= k
}

func main() {
	s := "waterrfetawx"
	k := 2
	if canMakePalindrome(s, k) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
