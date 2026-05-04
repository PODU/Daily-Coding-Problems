// Knight's tour counting via plain DFS backtracking from every start cell.
// Time: exponential (bounded by tour search); Space: O(N^2) visited grid + recursion.
package main

import "fmt"

var dx = [8]int{1, 1, -1, -1, 2, 2, -2, -2}
var dy = [8]int{2, -2, 2, -2, 1, -1, 1, -1}

func countTours(n int) int64 {
	visited := make([][]bool, n)
	for i := range visited {
		visited[i] = make([]bool, n)
	}
	target := n * n
	var total int64

	var dfs func(x, y, count int)
	dfs = func(x, y, count int) {
		if count == target {
			total++
			return
		}
		for d := 0; d < 8; d++ {
			nx, ny := x+dx[d], y+dy[d]
			if nx >= 0 && nx < n && ny >= 0 && ny < n && !visited[nx][ny] {
				visited[nx][ny] = true
				dfs(nx, ny, count+1)
				visited[nx][ny] = false
			}
		}
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			visited[i][j] = true
			dfs(i, j, 1)
			visited[i][j] = false
		}
	}
	return total
}

func main() {
	fmt.Println(countTours(1))
	fmt.Println(countTours(5))
}
