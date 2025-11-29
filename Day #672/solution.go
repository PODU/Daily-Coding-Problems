// Day 672: Maximum-weight top-to-bottom triangle path. Bottom-up DP folding rows.
// Time O(n^2) cells, Space O(n).
package main

import "fmt"

func maxPath(t [][]int) int {
	n := len(t)
	dp := make([]int, n)
	copy(dp, t[n-1])
	for i := n - 2; i >= 0; i-- {
		for j := 0; j <= i; j++ {
			best := dp[j]
			if dp[j+1] > best {
				best = dp[j+1]
			}
			dp[j] = t[i][j] + best
		}
	}
	return dp[0]
}

func main() {
	t := [][]int{{1}, {2, 3}, {1, 5, 1}}
	fmt.Println(maxPath(t)) // 9
}
