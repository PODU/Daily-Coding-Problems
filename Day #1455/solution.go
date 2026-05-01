// Day 1455: Maximum top-to-bottom path sum in a triangle. Bottom-up DP folding
// each row into the one above. Time O(n^2), Space O(n).
package main

import "fmt"

func maxPathSum(triangle [][]int) int {
	if len(triangle) == 0 {
		return 0
	}
	n := len(triangle)
	dp := make([]int, len(triangle[n-1]))
	copy(dp, triangle[n-1])
	for r := n - 2; r >= 0; r-- {
		for i := 0; i <= r; i++ {
			best := dp[i]
			if dp[i+1] > best {
				best = dp[i+1]
			}
			dp[i] = triangle[r][i] + best
		}
	}
	return dp[0]
}

func main() {
	triangle := [][]int{{1}, {2, 3}, {1, 5, 1}}
	fmt.Println(maxPathSum(triangle)) // 9  (1 -> 3 -> 5)
}
