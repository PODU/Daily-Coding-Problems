// Day 1333: Count right/down paths from top-left to bottom-right avoiding walls (1).
// DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. O(N*M) time, O(M) space.
package main

import "fmt"

func countPaths(g [][]int) int64 {
	n, m := len(g), len(g[0])
	dp := make([]int64, m)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if g[i][j] == 1 {
				dp[j] = 0
				continue
			}
			if i == 0 && j == 0 {
				dp[j] = 1
			} else {
				var up, left int64
				if i > 0 {
					up = dp[j]
				}
				if j > 0 {
					left = dp[j-1]
				}
				dp[j] = up + left
			}
		}
	}
	return dp[m-1]
}

func main() {
	g := [][]int{{0, 0, 1}, {0, 0, 1}, {1, 0, 0}}
	fmt.Println(countPaths(g)) // 2
}
