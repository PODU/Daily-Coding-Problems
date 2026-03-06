// Knight on board probability via DP over moves. prob(m,r,c)=avg of 8 neighbors.
// Time: O(k*64*8), Space: O(64).
package main

import (
	"fmt"
	"strconv"
)

func knightProbability(k, startR, startC int) float64 {
	dr := []int{-2, -2, -1, -1, 1, 1, 2, 2}
	dc := []int{-1, 1, -2, 2, -2, 2, -1, 1}
	prob := make([][]float64, 8)
	for i := range prob {
		prob[i] = make([]float64, 8)
		for j := range prob[i] {
			prob[i][j] = 1.0
		}
	}
	for m := 0; m < k; m++ {
		next := make([][]float64, 8)
		for i := range next {
			next[i] = make([]float64, 8)
		}
		for r := 0; r < 8; r++ {
			for c := 0; c < 8; c++ {
				s := 0.0
				for d := 0; d < 8; d++ {
					nr, nc := r+dr[d], c+dc[d]
					if nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
						s += prob[nr][nc]
					}
				}
				next[r][c] = s / 8.0
			}
		}
		prob = next
	}
	return prob[startR][startC]
}

func main() {
	ans := knightProbability(1, 0, 0)
	fmt.Println(strconv.FormatFloat(ans, 'g', -1, 64))
}
