// Day 635: Regular expression matching with '.' and '*' (full-string match).
// Approach: bottom-up DP; dp[i][j] = does s[i:] match p[j:].
// Time: O(m*n), Space: O(m*n).
package main

import "fmt"

func isMatch(s, p string) bool {
	m, n := len(s), len(p)
	dp := make([][]bool, m+1)
	for i := range dp {
		dp[i] = make([]bool, n+1)
	}
	dp[m][n] = true
	for i := m; i >= 0; i-- {
		for j := n - 1; j >= 0; j-- {
			first := i < m && (p[j] == s[i] || p[j] == '.')
			if j+1 < n && p[j+1] == '*' {
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
