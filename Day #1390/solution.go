// Max coins on a grid moving right/down via DP: dp[i][j]=m[i][j]+max(up,left). O(R*C) time/space.
package main

import "fmt"

func maxInt(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	m := [][]int{
		{0, 3, 1, 1},
		{2, 0, 0, 4},
		{1, 5, 3, 1},
	}
	R, C := len(m), len(m[0])
	dp := make([][]int, R)
	for i := range dp {
		dp[i] = make([]int, C)
	}
	for i := 0; i < R; i++ {
		for j := 0; j < C; j++ {
			best := 0
			if i > 0 {
				best = maxInt(best, dp[i-1][j])
			}
			if j > 0 {
				best = maxInt(best, dp[i][j-1])
			}
			if i == 0 && j == 0 {
				dp[i][j] = m[i][j]
			} else {
				dp[i][j] = m[i][j] + best
			}
		}
	}
	fmt.Println(dp[R-1][C-1]) // 12
}
