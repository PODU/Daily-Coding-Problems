// Day 529: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i] = min cuts for prefix i with parent pointers
// to reconstruct one optimal partition. Time O(n^2), space O(n^2).
package main

import (
	"fmt"
	"strings"
)

func minPalindromePartition(s string) []string {
	n := len(s)
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
	}
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			pal[i][j] = s[i] == s[j] && (j-i < 2 || pal[i+1][j-1])
		}
	}

	const INF = 1 << 30
	dp := make([]int, n+1)
	prev := make([]int, n+1)
	for i := range dp {
		dp[i] = INF
		prev[i] = -1
	}
	dp[0] = 0
	for j := 1; j <= n; j++ {
		for i := 0; i < j; i++ {
			if pal[i][j-1] && dp[i]+1 < dp[j] {
				dp[j] = dp[i] + 1
				prev[j] = i
			}
		}
	}

	var parts []string
	for j := n; j > 0; j = prev[j] {
		parts = append([]string{s[prev[j]:j]}, parts...)
	}
	return parts
}

func main() {
	s := "racecarannakayak"
	parts := minPalindromePartition(s)
	quoted := make([]string, len(parts))
	for i, p := range parts {
		quoted[i] = "\"" + p + "\""
	}
	fmt.Printf("[%s]\n", strings.Join(quoted, ", ")) // expected: ["racecar", "anna", "kayak"]
}
