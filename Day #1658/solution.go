// Max weight path top->bottom in triangle, bottom-up DP collapsing rows. O(n) space.
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
	fmt.Println(maxPath([][]int{{1}, {2, 3}, {1, 5, 1}}))
}
