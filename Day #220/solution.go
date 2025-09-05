// Day 220: Optimal coin-picking game (first player guaranteed max).
// Approach: interval DP. dp[i][j] = max you can collect from coins i..j vs optimal opponent.
// Time O(n^2), Space O(n^2).
package main

import "fmt"

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func maxCoins(v []int) int {
	n := len(v)
	if n == 0 {
		return 0
	}
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
		dp[i][i] = v[i]
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			innerLeft, innerMid, innerRight := 0, 0, 0
			if i+2 <= j {
				innerLeft = dp[i+2][j]
			}
			if i+1 <= j-1 {
				innerMid = dp[i+1][j-1]
			}
			if i <= j-2 {
				innerRight = dp[i][j-2]
			}
			takeI := v[i] + min(innerLeft, innerMid)
			takeJ := v[j] + min(innerMid, innerRight)
			dp[i][j] = max(takeI, takeJ)
		}
	}
	return dp[0][n-1]
}

func main() {
	fmt.Println(maxCoins([]int{8, 15, 3, 7})) // 22
}
