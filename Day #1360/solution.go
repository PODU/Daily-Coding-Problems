// Count Android unlock patterns of length N via DFS backtracking with a skip
// (midpoint) table; symmetry over corners/edges. Time O(N!) worst, Space O(N).
package main

import "fmt"

var skip [10][10]int
var visited [10]bool

func dfs(cur, remaining int) int {
	if remaining == 0 {
		return 1
	}
	visited[cur] = true
	count := 0
	for next := 1; next <= 9; next++ {
		if !visited[next] && (skip[cur][next] == 0 || visited[skip[cur][next]]) {
			count += dfs(next, remaining-1)
		}
	}
	visited[cur] = false
	return count
}

func countPatterns(n int) int {
	skip = [10][10]int{}
	skip[1][3], skip[3][1] = 2, 2
	skip[1][7], skip[7][1] = 4, 4
	skip[3][9], skip[9][3] = 6, 6
	skip[7][9], skip[9][7] = 8, 8
	skip[1][9], skip[9][1] = 5, 5
	skip[2][8], skip[8][2] = 5, 5
	skip[3][7], skip[7][3] = 5, 5
	skip[4][6], skip[6][4] = 5, 5
	corner := dfs(1, n-1)
	edge := dfs(2, n-1)
	center := dfs(5, n-1)
	return corner*4 + edge*4 + center
}

func main() {
	fmt.Println(countPatterns(4))
}
