// Day 838: Max coins moving only right/down through a grid.
// DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]).  Time O(R*C), Space O(R*C).
package main

import "fmt"

func maxCoins(grid [][]int) int {
	if len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	rows, cols := len(grid), len(grid[0])
	dp := make([][]int, rows)
	for i := range dp {
		dp[i] = make([]int, cols)
	}
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			best := 0
			if i > 0 && dp[i-1][j] > best {
				best = dp[i-1][j]
			}
			if j > 0 && dp[i][j-1] > best {
				best = dp[i][j-1]
			}
			dp[i][j] = grid[i][j] + best
		}
	}
	return dp[rows-1][cols-1]
}

func main() {
	matrix := [][]int{
		{0, 3, 1, 1},
		{2, 0, 0, 4},
		{1, 5, 3, 1},
	}
	fmt.Println(maxCoins(matrix))
}
