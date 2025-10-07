// Min coins via bottom-up DP. Returns (-1, false) if unreachable.
// Time: O(amount * |coins|), Space: O(amount).
package main

import "fmt"

func minCoins(coins []int, amount int) (int, bool) {
	const INF = 1 << 30
	dp := make([]int, amount+1)
	for i := range dp {
		dp[i] = INF
	}
	dp[0] = 0
	for a := 1; a <= amount; a++ {
		for _, c := range coins {
			if c <= a && dp[a-c]+1 < dp[a] {
				dp[a] = dp[a-c] + 1
			}
		}
	}
	if dp[amount] == INF {
		return -1, false
	}
	return dp[amount], true
}

func show(coins []int, amount int) {
	if r, ok := minCoins(coins, amount); ok {
		fmt.Println(r)
	} else {
		fmt.Println("null")
	}
}

func main() {
	show([]int{1, 5, 10}, 56)
	show([]int{5, 8}, 15)
}
