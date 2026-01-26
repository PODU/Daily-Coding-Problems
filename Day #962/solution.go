// Day 962: Count knight's tours on an N x N board (visit every square once).
// Approach: DFS backtracking from every start square. Time O(8^(N^2)) worst, Space O(N^2).
package main

import "fmt"

var n int
var dx = []int{1, 1, -1, -1, 2, 2, -2, -2}
var dy = []int{2, -2, 2, -2, 1, -1, 1, -1}

func dfs(vis [][]bool, x, y, count int) int64 {
	if count == n*n {
		return 1
	}
	var total int64
	for d := 0; d < 8; d++ {
		nx, ny := x+dx[d], y+dy[d]
		if nx >= 0 && nx < n && ny >= 0 && ny < n && !vis[nx][ny] {
			vis[nx][ny] = true
			total += dfs(vis, nx, ny, count+1)
			vis[nx][ny] = false
		}
	}
	return total
}

func countTours(sz int) int64 {
	n = sz
	var total int64
	for i := 0; i < sz; i++ {
		for j := 0; j < sz; j++ {
			vis := make([][]bool, sz)
			for k := range vis {
				vis[k] = make([]bool, sz)
			}
			vis[i][j] = true
			total += dfs(vis, i, j, 1)
		}
	}
	return total
}

func main() {
	fmt.Println(countTours(5)) // 1728
}
