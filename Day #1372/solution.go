// Shortest path on grid with walls via BFS. Time O(M*N), Space O(M*N).
// Returns -1 ("null") if unreachable.
package main

import "fmt"

func shortestPath(grid [][]bool, start, end [2]int) int {
	m, n := len(grid), len(grid[0])
	if grid[start[0]][start[1]] || grid[end[0]][end[1]] {
		return -1
	}
	dist := make([][]int, m)
	for i := range dist {
		dist[i] = make([]int, n)
		for j := range dist[i] {
			dist[i][j] = -1
		}
	}
	dist[start[0]][start[1]] = 0
	q := [][2]int{start}
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		r, c := cur[0], cur[1]
		if r == end[0] && c == end[1] {
			return dist[r][c]
		}
		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]
			if nr >= 0 && nr < m && nc >= 0 && nc < n && !grid[nr][nc] && dist[nr][nc] == -1 {
				dist[nr][nc] = dist[r][c] + 1
				q = append(q, [2]int{nr, nc})
			}
		}
	}
	return -1
}

func main() {
	f, t := false, true
	board := [][]bool{
		{f, f, f, f},
		{t, t, f, t},
		{f, f, f, f},
		{f, f, f, f},
	}
	res := shortestPath(board, [2]int{3, 0}, [2]int{0, 0})
	if res == -1 {
		fmt.Println("null")
	} else {
		fmt.Println(res)
	}
}
