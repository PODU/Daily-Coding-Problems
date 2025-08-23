// Day 158: Count paths (right/down only) avoiding walls. DP: dp[j] = ways into a
// free cell from top+left; walls contribute 0. Time O(N*M), Space O(M).
package main

import "fmt"

func countPaths(grid [][]int) int64 {
	n, m := len(grid), len(grid[0])
	dp := make([]int64, m)
	if grid[0][0] == 0 {
		dp[0] = 1
	}
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if grid[i][j] == 1 {
				dp[j] = 0
			} else if j > 0 {
				dp[j] += dp[j-1]
			}
		}
	}
	return dp[m-1]
}

func main() {
	grid := [][]int{
		{0, 0, 1},
		{0, 0, 1},
		{1, 0, 0},
	}
	fmt.Println(countPaths(grid)) // 2
}
