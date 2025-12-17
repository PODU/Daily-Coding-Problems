// Day 754: Optimal coin game (interval DP / minimax).
// dp[i][j] = max value first player guarantees from coins[i..j].
// Time: O(n^2), Space: O(n^2).
package main

import "fmt"

func maxCoins(v []int) int {
	n := len(v)
	if n == 0 {
		return 0
	}
	pre := make([]int, n+1)
	for i := 0; i < n; i++ {
		pre[i+1] = pre[i] + v[i]
	}
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
		dp[i][i] = v[i]
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			takeLeft := v[i] + (pre[j+1] - pre[i+1]) - dp[i+1][j]
			takeRight := v[j] + (pre[j] - pre[i]) - dp[i][j-1]
			if takeLeft > takeRight {
				dp[i][j] = takeLeft
			} else {
				dp[i][j] = takeRight
			}
		}
	}
	return dp[0][n-1]
}

func main() {
	coins := []int{8, 15, 3, 7}
	fmt.Println(maxCoins(coins)) // 22
}
