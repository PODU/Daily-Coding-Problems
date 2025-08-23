// Day 156: Min perfect squares summing to n via DP. dp[i] = min over squares
// j*j<=i of dp[i-j*j]+1. Time O(n*sqrt(n)), Space O(n).
package main

import "fmt"

func numSquares(n int) int {
	dp := make([]int, n+1)
	for i := 1; i <= n; i++ {
		dp[i] = 1 << 30
		for j := 1; j*j <= i; j++ {
			if dp[i-j*j]+1 < dp[i] {
				dp[i] = dp[i-j*j] + 1
			}
		}
	}
	return dp[n]
}

func main() {
	fmt.Println(numSquares(13)) // 2
	fmt.Println(numSquares(27)) // 3
}
