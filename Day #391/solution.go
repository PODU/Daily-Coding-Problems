// Day 391: Longest common contiguous subsequence (substring) of two histories.
// DP on suffix-run lengths. Time O(n*m), Space O(n*m).
package main

import "fmt"

func longestCommon(a, b []string) []string {
	n, m := len(a), len(b)
	dp := make([][]int, n+1)
	for i := range dp {
		dp[i] = make([]int, m+1)
	}
	best, endI := 0, 0
	for i := 1; i <= n; i++ {
		for j := 1; j <= m; j++ {
			if a[i-1] == b[j-1] {
				dp[i][j] = dp[i-1][j-1] + 1
				if dp[i][j] > best {
					best, endI = dp[i][j], i
				}
			}
		}
	}
	return a[endI-best : endI]
}

func main() {
	user1 := []string{"/home", "/register", "/login", "/user", "/one", "/two"}
	user2 := []string{"/home", "/red", "/login", "/user", "/one", "/pink"}
	fmt.Println(longestCommon(user1, user2))
}
