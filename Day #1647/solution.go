// Coins-in-a-row: dp[i][j] = max value first-to-move guarantees from coins[i..j],
// dp[i][j]=max(v[i]+min(dp[i+2][j],dp[i+1][j-1]), v[j]+min(dp[i+1][j-1],dp[i][j-2])). Time/Space O(n^2).
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

func maxGuaranteed(v []int) int {
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
			a := 0
			if i+2 <= j {
				a = dp[i+2][j]
			}
			b := 0
			if i+1 <= j-1 {
				b = dp[i+1][j-1]
			}
			c := 0
			if i <= j-2 {
				c = dp[i][j-2]
			}
			takeFirst := v[i] + minInt(a, b)
			takeLast := v[j] + minInt(b, c)
			dp[i][j] = maxInt(takeFirst, takeLast)
		}
	}
	return dp[0][n-1]
}

func main() {
	coins := []int{3, 9, 1, 2}
	fmt.Println(maxGuaranteed(coins))
}
