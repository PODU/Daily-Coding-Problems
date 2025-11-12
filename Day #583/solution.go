// Grid DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]). Time O(R*C), Space O(R*C).
package main

import "fmt"

func maxi(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func maxCoins(grid [][]int) int {
	R, C := len(grid), len(grid[0])
	dp := make([][]int, R)
	for i := range dp {
		dp[i] = make([]int, C)
	}
	for i := 0; i < R; i++ {
		for j := 0; j < C; j++ {
			best := 0
			if i > 0 {
				best = maxi(best, dp[i-1][j])
			}
			if j > 0 {
				best = maxi(best, dp[i][j-1])
			}
			if i == 0 && j == 0 {
				dp[i][j] = grid[i][j]
			} else {
				dp[i][j] = grid[i][j] + best
			}
		}
	}
	return dp[R-1][C-1]
}

func main() {
	grid := [][]int{{0, 3, 1, 1}, {2, 0, 0, 4}, {1, 5, 3, 1}}
	result := maxCoins(grid)
	if result != 12 {
		panic("expected 12")
	}
	fmt.Printf("The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = %d coins.\n", result)
}
