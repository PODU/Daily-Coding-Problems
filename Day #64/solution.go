// Count open knight's tours on N x N board via backtracking DFS from every start.
// Time exponential, Space O(N^2).
package main

import "fmt"

var moves = [8][2]int{{-2, -1}, {-2, 1}, {-1, -2}, {-1, 2}, {1, -2}, {1, 2}, {2, -1}, {2, 1}}

func countTours(n int) int64 {
	vis := make([][]bool, n)
	for i := range vis {
		vis[i] = make([]bool, n)
	}
	var dfs func(r, c, count int) int64
	dfs = func(r, c, count int) int64 {
		if count == n*n {
			return 1
		}
		var res int64
		for _, mv := range moves {
			nr, nc := r+mv[0], c+mv[1]
			if nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr][nc] {
				vis[nr][nc] = true
				res += dfs(nr, nc, count+1)
				vis[nr][nc] = false
			}
		}
		return res
	}
	var total int64
	for r := 0; r < n; r++ {
		for c := 0; c < n; c++ {
			vis[r][c] = true
			total += dfs(r, c, 1)
			vis[r][c] = false
		}
	}
	return total
}

func main() {
	fmt.Println(countTours(5))
}
