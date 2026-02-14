// Coin game DP: dp[i][j] = max guaranteed for current player on coins[i..j]. O(n^2) time/space.
package main

import "fmt"

func minInt(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func maxInt(a, b int) int {
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
	for i := 0; i < n; i++ {
		dp[i][i] = v[i]
	}
	for length := 2; length <= n; length++ {
		for i := 0; i <= n-length; i++ {
			j := i + length - 1
			a := 0
			if i+2 <= j {
				a = dp[i+2][j]
			}
			b := 0
			if i+1 <= j-1 {
				b = dp[i+1][j-1]
			}
			takeI := v[i] + minInt(a, b)

			c := 0
			if i+1 <= j-1 {
				c = dp[i+1][j-1]
			}
			d := 0
			if i <= j-2 {
				d = dp[i][j-2]
			}
			takeJ := v[j] + minInt(c, d)

			dp[i][j] = maxInt(takeI, takeJ)
		}
	}
	return dp[0][n-1]
}

func main() {
	a := []int{8, 15, 3, 7}
	fmt.Printf("Max guaranteed: %d\n", coinGame(a))
	b := []int{2, 2, 2, 2}
	fmt.Printf("Max guaranteed: %d\n", coinGame(b))
}
