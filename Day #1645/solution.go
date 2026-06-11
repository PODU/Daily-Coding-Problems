// Regex full-match with '.' and '*' via DP: dp[i][j] = s[0..i) matches p[0..j).
// '*' uses zero-copy (dp[i][j-2]) or one-more (prev char match). Time/Space O(m*n).
package main

import "fmt"

func isMatch(s, p string) bool {
	m, n := len(s), len(p)
	dp := make([][]bool, m+1)
	for i := range dp {
		dp[i] = make([]bool, n+1)
	}
	dp[0][0] = true
	for j := 1; j <= n; j++ {
		if p[j-1] == '*' && j >= 2 {
			dp[0][j] = dp[0][j-2]
		}
	}
	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if p[j-1] == '*' {
				dp[i][j] = j >= 2 && dp[i][j-2]
				if j >= 2 && (p[j-2] == '.' || p[j-2] == s[i-1]) {
					dp[i][j] = dp[i][j] || dp[i-1][j]
				}
			} else if p[j-1] == '.' || p[j-1] == s[i-1] {
				dp[i][j] = dp[i-1][j-1]
			}
		}
	}
	return dp[m][n]
}

func main() {
	fmt.Println(isMatch("ray", "ra."))
	fmt.Println(isMatch("raymond", "ra."))
	fmt.Println(isMatch("chat", ".*at"))
	fmt.Println(isMatch("chats", ".*at"))
}
