// Day 304: Probability knight stays on board after k random moves. DP over board.
// Time O(k*N^2*8), space O(N^2).
package main

import "fmt"

func knightProb(N, k, sr, sc int) float64 {
	dp := make([][]float64, N)
	for i := range dp {
		dp[i] = make([]float64, N)
	}
	dp[sr][sc] = 1.0
	dr := []int{1, 1, -1, -1, 2, 2, -2, -2}
	dc := []int{2, -2, 2, -2, 1, -1, 1, -1}
	for step := 0; step < k; step++ {
		nd := make([][]float64, N)
		for i := range nd {
			nd[i] = make([]float64, N)
		}
		for r := 0; r < N; r++ {
			for c := 0; c < N; c++ {
				if dp[r][c] > 0 {
					for d := 0; d < 8; d++ {
						nr, nc := r+dr[d], c+dc[d]
						if nr >= 0 && nr < N && nc >= 0 && nc < N {
							nd[nr][nc] += dp[r][c] / 8.0
						}
					}
				}
			}
		}
		dp = nd
	}
	tot := 0.0
	for _, row := range dp {
		for _, v := range row {
			tot += v
		}
	}
	return tot
}

func main() {
	fmt.Println(knightProb(8, 1, 0, 0)) // 0.25
}
