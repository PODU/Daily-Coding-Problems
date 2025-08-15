// Day 122: Max coins from top-left to bottom-right moving right/down.
// DP O(R*C) time and space, with path reconstruction (prefer left on ties).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	g := [][]int{{0, 3, 1, 1}, {2, 0, 0, 4}, {1, 5, 3, 1}}
	R, C := len(g), len(g[0])
	dp := make([][]int, R)
	for i := range dp {
		dp[i] = make([]int, C)
	}
	for i := 0; i < R; i++ {
		for j := 0; j < C; j++ {
			best := 0
			if i == 0 && j == 0 {
				best = 0
			} else if i == 0 {
				best = dp[i][j-1]
			} else if j == 0 {
				best = dp[i-1][j]
			} else if dp[i-1][j] > dp[i][j-1] {
				best = dp[i-1][j]
			} else {
				best = dp[i][j-1]
			}
			dp[i][j] = g[i][j] + best
		}
	}
	var path []int
	i, j := R-1, C-1
	for i > 0 || j > 0 {
		path = append(path, g[i][j])
		if i == 0 {
			j--
		} else if j == 0 {
			i--
		} else if dp[i-1][j] > dp[i][j-1] {
			i--
		} else {
			j--
		}
	}
	path = append(path, g[0][0])
	for l, r := 0, len(path)-1; l < r; l, r = l+1, r-1 {
		path[l], path[r] = path[r], path[l]
	}
	parts := make([]string, len(path))
	for idx, v := range path {
		parts[idx] = strconv.Itoa(v)
	}
	fmt.Printf("The most we can collect is %s = %d coins.\n", strings.Join(parts, " + "), dp[R-1][C-1])
}
