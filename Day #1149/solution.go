// Day 1149: Number of islands - 4-directional flood fill.
// Iterative DFS marks visited land. Time O(R*C), Space O(R*C).
package main

import "fmt"

func numIslands(grid [][]int) int {
	rows := len(grid)
	if rows == 0 {
		return 0
	}
	cols := len(grid[0])
	g := make([][]int, rows)
	for i := range grid {
		g[i] = append([]int{}, grid[i]...)
	}
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	count := 0
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if g[r][c] == 1 {
				count++
				stack := [][2]int{{r, c}}
				g[r][c] = 0
				for len(stack) > 0 {
					cur := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					for _, d := range dirs {
						nx, ny := cur[0]+d[0], cur[1]+d[1]
						if nx >= 0 && ny >= 0 && nx < rows && ny < cols && g[nx][ny] == 1 {
							g[nx][ny] = 0
							stack = append(stack, [2]int{nx, ny})
						}
					}
				}
			}
		}
	}
	return count
}

func main() {
	grid := [][]int{
		{1, 0, 0, 0, 0}, {0, 0, 1, 1, 0}, {0, 1, 1, 0, 0},
		{0, 0, 0, 0, 0}, {1, 1, 0, 0, 1}, {1, 1, 0, 0, 1}}
	fmt.Println(numIslands(grid)) // 4
}
