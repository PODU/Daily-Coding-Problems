// Grid DP: max coins from top-left to bottom-right moving right/down only.
// dp[j] = grid + max(top, left). Time O(m*n), Space O(n).
package main

import "fmt"

func main() {
	grid := [][]int{
		{0, 3, 1, 1},
		{2, 0, 0, 4},
		{1, 5, 3, 1},
	}
	m, n := len(grid), len(grid[0])
	dp := make([]int, n)
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			var best int
			if i == 0 && j == 0 {
				best = 0
			} else if i == 0 {
				best = dp[j-1]
			} else if j == 0 {
				best = dp[j]
			} else if dp[j] > dp[j-1] {
				best = dp[j]
			} else {
				best = dp[j-1]
			}
			dp[j] = grid[i][j] + best
		}
	}
	fmt.Println(dp[n-1])
}
