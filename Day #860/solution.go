// Day 860: Regex matching with '.' and '*'.
// Approach: bottom-up DP, dp[i][j] = does s[i:] match p[j:].
// Time: O(n*m), Space: O(n*m).
package main

import "fmt"

func isMatch(s, p string) bool {
	n, m := len(s), len(p)
	dp := make([][]bool, n+1)
	for i := range dp {
		dp[i] = make([]bool, m+1)
	}
	dp[n][m] = true
	for i := n; i >= 0; i-- {
		for j := m - 1; j >= 0; j-- {
			first := i < n && (p[j] == s[i] || p[j] == '.')
			if j+1 < m && p[j+1] == '*' {
				dp[i][j] = dp[i][j+2] || (first && dp[i+1][j])
			} else {
				dp[i][j] = first && dp[i+1][j+1]
			}
		}
	}
	return dp[0][0]
}

func main() {
	fmt.Println(isMatch("ray", "ra."))     // true
	fmt.Println(isMatch("raymond", "ra.")) // false
	fmt.Println(isMatch("chat", ".*at"))   // true
	fmt.Println(isMatch("chats", ".*at"))  // false
}
