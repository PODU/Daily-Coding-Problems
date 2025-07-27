// Min-insertion palindrome, lexicographically earliest. DP over substrings; O(n^2) states, O(n^3) overall.
package main

import "fmt"

func solve(s string) string {
	n := len(s)
	if n == 0 {
		return ""
	}
	dp := make([][]string, n)
	for i := range dp {
		dp[i] = make([]string, n)
	}
	for i := 0; i < n; i++ {
		dp[i][i] = string(s[i])
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			if s[i] == s[j] {
				inner := ""
				if i+1 <= j-1 {
					inner = dp[i+1][j-1]
				}
				dp[i][j] = string(s[i]) + inner + string(s[j])
			} else {
				c1 := string(s[i]) + dp[i+1][j] + string(s[i])
				c2 := string(s[j]) + dp[i][j-1] + string(s[j])
				if len(c1) < len(c2) {
					dp[i][j] = c1
				} else if len(c2) < len(c1) {
					dp[i][j] = c2
				} else if c1 <= c2 {
					dp[i][j] = c1
				} else {
					dp[i][j] = c2
				}
			}
		}
	}
	return dp[0][n-1]
}

func main() {
	fmt.Printf("\"%s\"\n", solve("race"))
}
