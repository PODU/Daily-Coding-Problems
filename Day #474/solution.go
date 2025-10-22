// Min coins via DP over amounts (optimal for arbitrary denominations).
// Greedy also works for canonical US coins {1,5,10,25}. Time: O(n*k), Space: O(n).
package main

import "fmt"

func minCoins(n int, coins []int) int {
	const INF = int(^uint(0) >> 1)
	dp := make([]int, n+1)
	for i := 1; i <= n; i++ {
		dp[i] = INF
	}
	for a := 1; a <= n; a++ {
		for _, c := range coins {
			if c <= a && dp[a-c] != INF {
				if dp[a-c]+1 < dp[a] {
					dp[a] = dp[a-c] + 1
				}
			}
		}
	}
	return dp[n]
}

func main() {
	coins := []int{1, 5, 10, 25}
	n := 16
	fmt.Println(minCoins(n, coins))
}
