// Day 861: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over moves; dp[r][c] = prob of being on this cell, spread 1/8 each move.
// Time: O(k * N^2), Space: O(N^2).
package main

import "fmt"

var dr = []int{-2, -2, -1, -1, 1, 1, 2, 2}
var dc = []int{-1, 1, -2, 2, -2, 2, -1, 1}

func knightProbability(n, k, sr, sc int) float64 {
	dp := make([][]float64, n)
	for i := range dp {
		dp[i] = make([]float64, n)
	}
	dp[sr][sc] = 1.0
	for step := 0; step < k; step++ {
		nx := make([][]float64, n)
		for i := range nx {
			nx[i] = make([]float64, n)
		}
		for r := 0; r < n; r++ {
			for c := 0; c < n; c++ {
				if dp[r][c] > 0 {
					for d := 0; d < 8; d++ {
						nr, nc := r+dr[d], c+dc[d]
						if nr >= 0 && nr < n && nc >= 0 && nc < n {
							nx[nr][nc] += dp[r][c] / 8.0
						}
					}
				}
			}
		}
		dp = nx
	}
	total := 0.0
	for _, row := range dp {
		for _, v := range row {
			total += v
		}
	}
	return total
}

func main() {
	fmt.Println(knightProbability(8, 1, 0, 0)) // 0.25
	fmt.Println(knightProbability(8, 2, 0, 0))
}
