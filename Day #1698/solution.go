// Deduce coin denominations from ways[] via incremental unbounded-knapsack DP.
// Time O(N^2), Space O(N).
package main

import "fmt"

func findDenominations(ways []int) []int {
	n := len(ways)
	dp := make([]int64, n)
	dp[0] = 1
	coins := []int{}
	for i := 1; i < n; i++ {
		if int64(ways[i]) != dp[i] {
			coins = append(coins, i)
			for j := i; j < n; j++ {
				dp[j] += dp[j-i]
			}
		}
	}
	return coins
}

func main() {
	ways := []int{1, 0, 1, 1, 2}
	fmt.Println(findDenominations(ways))
}
