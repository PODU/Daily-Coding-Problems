// Count knight's tours on NxN via DFS backtracking from every start cell.
// Worst-case exponential; fine for small N (N=5 -> 1728).
package main

import "fmt"

var dr = []int{-2, -2, -1, -1, 1, 1, 2, 2}
var dc = []int{-1, 1, -2, 2, -2, 2, -1, 1}

func dfs(n, r, c, count int, vis [][]bool) int64 {
	if count == n*n {
		return 1
	}
	var total int64 = 0
	for k := 0; k < 8; k++ {
		nr, nc := r+dr[k], c+dc[k]
		if nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr][nc] {
			vis[nr][nc] = true
			total += dfs(n, nr, nc, count+1, vis)
			vis[nr][nc] = false
		}
	}
	return total
}

func countTours(n int) int64 {
	var total int64 = 0
	for r := 0; r < n; r++ {
		for c := 0; c < n; c++ {
			vis := make([][]bool, n)
			for i := range vis {
				vis[i] = make([]bool, n)
			}
			vis[r][c] = true
			total += dfs(n, r, c, 1, vis)
		}
	}
	return total
}

func main() {
	fmt.Println(countTours(5)) // 1728
}
