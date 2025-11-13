// Day 592: Count islands in a binary matrix.
// Approach: DFS flood-fill over each unvisited land cell (4-directional).
// Time: O(R*C), Space: O(R*C) recursion stack.
package main

import "fmt"

func dfs(g [][]int, r, c int) {
	if r < 0 || c < 0 || r >= len(g) || c >= len(g[0]) || g[r][c] != 1 {
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
	for r := range g {
		for c := range g[0] {
			if g[r][c] == 1 {
				count++
				dfs(g, r, c)
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
