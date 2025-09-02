// Day 201: Maximum weight path in a triangle.
// Bottom-up DP: each cell becomes its value + max of the two children below.
// Time: O(n^2), Space: O(n).
package main

import "fmt"

func maxPath(t [][]int) int {
	n := len(t)
	dp := append([]int(nil), t[n-1]...)
	for r := n - 2; r >= 0; r-- {
		for c := 0; c <= r; c++ {
			best := dp[c]
			if dp[c+1] > best {
				best = dp[c+1]
			}
			dp[c] = t[r][c] + best
		}
	}
	return dp[0]
}

func main() {
	fmt.Println(maxPath([][]int{{1}, {2, 3}, {1, 5, 1}})) // 9
}
