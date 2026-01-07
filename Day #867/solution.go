// Day 867: Maximum weight path from top to bottom of a triangle.
// Approach: bottom-up DP, fold each row into the one above using max of adjacent.
// Time: O(n^2), Space: O(n).
package main

import "fmt"

func maxPath(t [][]int) int {
	dp := make([]int, len(t[len(t)-1]))
	copy(dp, t[len(t)-1])
	for i := len(t) - 2; i >= 0; i-- {
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
