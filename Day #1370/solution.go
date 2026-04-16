// Can form palindrome by deleting <= k chars: min deletions = n - LPS(s).
// LPS via DP. Time O(n^2), Space O(n^2).
package main

import "fmt"

func lps(s string) int {
	n := len(s)
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
		dp[i][i] = 1
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			if s[i] == s[j] {
				if length == 2 {
					dp[i][j] = 2
				} else {
					dp[i][j] = dp[i+1][j-1] + 2
				}
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
	if canMakePalindrome("waterrfetawx", 2) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
