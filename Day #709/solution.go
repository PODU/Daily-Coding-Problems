// Day 709: Count paths top-left to bottom-right moving right/down, avoiding walls.
// DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. Time O(N*M), space O(M).
package main

import "fmt"

func countPaths(g [][]int) int64 {
	n, m := len(g), len(g[0])
	dp := make([]int64, m)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if g[i][j] == 1 {
				dp[j] = 0
			} else if i == 0 && j == 0 {
				dp[j] = 1
			} else {
				left := int64(0)
				if j > 0 {
					left = dp[j-1]
				}
				dp[j] = left + dp[j]
			}
		}
	}
	return dp[m-1]
}

func main() {
	grid := [][]int{{0, 0, 1}, {0, 0, 1}, {1, 0, 0}}
	fmt.Println(countPaths(grid))
}
