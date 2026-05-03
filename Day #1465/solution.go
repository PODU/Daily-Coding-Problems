// Optimal coin game via interval DP. dp[i][j] = best score first player can guarantee on coins[i..j].
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

func coinGame(v []int) int {
	n := len(v)
	if n == 0 {
		return 0
	}
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
	}
	for length := 1; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			a, b, c := 0, 0, 0
			if i+2 <= j {
				a = dp[i+2][j]
			}
			if i+1 <= j-1 {
				b = dp[i+1][j-1]
			}
			if i <= j-2 {
				c = dp[i][j-2]
			}
			takeFirst := v[i] + min(a, b)
			takeLast := v[j] + min(b, c)
			dp[i][j] = max(takeFirst, takeLast)
		}
	}
	return dp[0][n-1]
}

func main() {
	fmt.Println(coinGame([]int{8, 15, 3, 7}))
	fmt.Println(coinGame([]int{2, 2, 2, 2}))
}
