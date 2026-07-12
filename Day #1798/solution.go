// Grid shortest path via BFS over walkable cells. Time O(M*N), Space O(M*N).
// Walls are booleans (true=wall). Returns -1 if no path (null semantics).
package main

import "fmt"

func shortestPath(grid [][]bool, start, end [2]int) int {
	m, n := len(grid), len(grid[0])
	if grid[start[0]][start[1]] || grid[end[0]][end[1]] {
		return -1
	}
	visited := make([][]bool, m)
	for i := range visited {
		visited[i] = make([]bool, n)
	}
	queue := [][2]int{start}
	visited[start[0]][start[1]] = true
	steps := 0
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for len(queue) > 0 {
		next := [][2]int{}
		for _, cur := range queue {
			if cur == end {
				return steps
			}
			for _, d := range dirs {
				nr, nc := cur[0]+d[0], cur[1]+d[1]
				if nr >= 0 && nr < m && nc >= 0 && nc < n && !visited[nr][nc] && !grid[nr][nc] {
					visited[nr][nc] = true
					next = append(next, [2]int{nr, nc})
				}
			}
		}
		queue = next
		steps++
	}
	return -1
}

func main() {
	const F, T = false, true
	grid := [][]bool{
		{F, F, F, F},
		{T, T, F, T},
		{F, F, F, F},
		{F, F, F, F},
	}
	fmt.Println(shortestPath(grid, [2]int{3, 0}, [2]int{0, 0})) // 7
}
