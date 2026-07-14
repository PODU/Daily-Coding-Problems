// Knight-on-board probability after k random moves: DP over board states.
// dp[r][c] = prob of being on (r,c); each move spreads 1/8 to each target.
// Time: O(k*64*8). Space: O(64).
package main

import "fmt"

var dr = []int{-2, -2, -1, -1, 1, 1, 2, 2}
var dc = []int{-1, 1, -2, 2, -2, 2, -1, 1}

func knightProbability(n, k, r0, c0 int) float64 {
	dp := make([][]float64, n)
	for i := range dp {
		dp[i] = make([]float64, n)
	}
	dp[r0][c0] = 1.0
	for m := 0; m < k; m++ {
		nxt := make([][]float64, n)
		for i := range nxt {
			nxt[i] = make([]float64, n)
		}
		for r := 0; r < n; r++ {
			for c := 0; c < n; c++ {
				if dp[r][c] == 0 {
					continue
				}
				for d := 0; d < 8; d++ {
					nr, nc := r+dr[d], c+dc[d]
					if nr >= 0 && nr < n && nc >= 0 && nc < n {
						nxt[nr][nc] += dp[r][c] / 8.0
					}
				}
			}
		}
		dp = nxt
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
	// corner (0,0), k=1 -> 2/8 = 0.25
	fmt.Println(knightProbability(8, 1, 0, 0)) // 0.25
}
