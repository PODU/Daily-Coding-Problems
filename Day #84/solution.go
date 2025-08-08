// Day 84: Count islands (connected groups of 1s) via iterative DFS flood fill.
// Time O(rows*cols), Space O(rows*cols).
package main

import "fmt"

func numIslands(grid [][]int) int {
	g := make([][]int, len(grid))
	for i := range grid {
		g[i] = append([]int(nil), grid[i]...)
	}
	rows, cols := len(g), len(g[0])
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	count := 0
	for sr := 0; sr < rows; sr++ {
		for sc := 0; sc < cols; sc++ {
			if g[sr][sc] == 1 {
				count++
				stack := [][2]int{{sr, sc}}
				g[sr][sc] = 0
				for len(stack) > 0 {
					cur := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					for _, d := range dirs {
						nr, nc := cur[0]+d[0], cur[1]+d[1]
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
	fmt.Println(numIslands(grid)) // 4
}
