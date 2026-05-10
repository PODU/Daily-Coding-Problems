// Day 1500: Number of islands via iterative DFS flood fill (4-directional).
// Time O(R*C), Space O(R*C).
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
		g[i] = append([]int(nil), grid[i]...)
	}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	count := 0
	for sr := 0; sr < rows; sr++ {
		for sc := 0; sc < cols; sc++ {
			if g[sr][sc] == 1 {
				count++
				stack := [][2]int{{sr, sc}}
				g[sr][sc] = 0
				for len(stack) > 0 {
					cell := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					for _, d := range dirs {
						nr, nc := cell[0]+d[0], cell[1]+d[1]
						if nr >= 0 && nr < rows && nc >= 0 && nc < cols && g[nr][nc] == 1 {
							g[nr][nc] = 0
							stack = append(stack, [2]int{nr, nc})
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
		{1, 0, 0, 0, 0},
		{0, 0, 1, 1, 0},
		{0, 1, 1, 0, 0},
		{0, 0, 0, 0, 0},
		{1, 1, 0, 0, 1},
		{1, 1, 0, 0, 1},
	}
	fmt.Println(numIslands(grid))
}
