// Day 121: Can form palindrome by deleting at most k chars.
// Min deletions = n - LongestPalindromicSubsequence. DP O(n^2) time, O(n^2) space.
package main

import "fmt"

func lps(s string) int {
	n := len(s)
	if n == 0 {
		return 0
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
				add := 0
				if length > 2 {
					add = dp[i+1][j-1]
				}
				dp[i][j] = 2 + add
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
		fmt.Println("You could delete f and x to get 'waterretaw'.")
	} else {
		fmt.Println("Not possible")
	}
}
