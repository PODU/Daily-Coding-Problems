// Recover coin denominations from change-ways array A (unbounded coin change).
// dp starts {1,0,...}; if A[i] != dp[i], i is a coin -> unbounded-knapsack update. O(N^2) time, O(N) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func recoverCoins(A []int) []int {
	n := len(A)
	dp := make([]int, n)
	dp[0] = 1
	coins := []int{}
	for i := 1; i < n; i++ {
		if A[i] != dp[i] {
			coins = append(coins, i)
			for v := i; v < n; v++ {
				dp[v] += dp[v-i]
			}
		}
	}
	return coins
}

func formatList(xs []int) string {
	n := len(xs)
	parts := make([]string, n)
	for i, x := range xs {
		if i == n-1 && n > 1 {
			parts[i] = "and " + strconv.Itoa(x)
		} else {
			parts[i] = strconv.Itoa(x)
		}
	}
	return strings.Join(parts, ", ")
}

func main() {
	A := []int{1, 0, 1, 1, 2}
	fmt.Println(formatList(recoverCoins(A))) // 2, 3, and 4
}
