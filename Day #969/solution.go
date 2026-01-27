// Day 969: Count islands (4-connected groups of 1s) in a binary matrix.
// Approach: DFS flood fill from each unvisited land cell. Time O(R*C), Space O(R*C).
package main

import "fmt"

func dfs(g [][]int, r, c int) {
	if r < 0 || r >= len(g) || c < 0 || c >= len(g[0]) || g[r][c] == 0 {
		return
	}
	g[r][c] = 0
	dfs(g, r+1, c)
	dfs(g, r-1, c)
	dfs(g, r, c+1)
	dfs(g, r, c-1)
}

func numIslands(grid [][]int) int {
	g := make([][]int, len(grid))
	for i := range grid {
		g[i] = append([]int(nil), grid[i]...)
	}
	count := 0
	for i := range g {
		for j := range g[0] {
			if g[i][j] == 1 {
				count++
				dfs(g, i, j)
			}
		}
	}
	return count
}

func main() {
	grid := [][]int{
		{1, 0, 0, 0, 0},
		{0, 0, 1, 1, 0},
		{0, 1, 1, 0, 0},
		{0, 0, 0, 0, 0},
		{1, 1, 0, 0, 1},
		{1, 1, 0, 0, 1},
	}
	fmt.Println(numIslands(grid)) // 4
}
