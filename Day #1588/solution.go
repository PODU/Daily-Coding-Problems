// Count right/down paths in a grid with walls. DP: dp[i][j] = paths to cell (0 if wall).
// Time: O(N*M); Space: O(N*M).
package main

import "fmt"

func countPaths(grid [][]int) int64 {
	n, m := len(grid), len(grid[0])
	if grid[0][0] == 1 || grid[n-1][m-1] == 1 {
		return 0
	}
	dp := make([][]int64, n)
	for i := range dp {
		dp[i] = make([]int64, m)
	}
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if grid[i][j] == 1 {
				dp[i][j] = 0
				continue
			}
			if i == 0 && j == 0 {
				dp[i][j] = 1
				continue
			}
			var up, left int64
			if i > 0 {
				up = dp[i-1][j]
			}
			if j > 0 {
				left = dp[i][j-1]
			}
			dp[i][j] = up + left
		}
	}
	return dp[n-1][m-1]
}

func main() {
	grid := [][]int{{0, 0, 1}, {0, 0, 1}, {1, 0, 0}}
	fmt.Println(countPaths(grid))
}
