// Day 703: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over board cells; each move spreads prob/8 to valid targets.
// Time O(k * N^2 * 8), Space O(N^2).
package main

import "fmt"

func knightProbability(n, k, r, c int) float64 {
	dr := []int{1, 1, -1, -1, 2, 2, -2, -2}
	dc := []int{2, -2, 2, -2, 1, -1, 1, -1}
	dp := make([][]float64, n)
	for i := range dp {
		dp[i] = make([]float64, n)
	}
	dp[r][c] = 1.0
	for step := 0; step < k; step++ {
		nd := make([][]float64, n)
		for i := range nd {
			nd[i] = make([]float64, n)
		}
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				if dp[i][j] > 0 {
					for m := 0; m < 8; m++ {
						ni, nj := i+dr[m], j+dc[m]
						if ni >= 0 && ni < n && nj >= 0 && nj < n {
							nd[ni][nj] += dp[i][j] / 8.0
						}
					}
				}
			}
		}
		dp = nd
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
	fmt.Println(knightProbability(8, 2, 0, 0)) // 0.1875
}
