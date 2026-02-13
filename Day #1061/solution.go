// Shortest path on a boolean grid (true=wall) via BFS in 4 directions.
// Time: O(M*N), Space: O(M*N). Returns -1 (null) if unreachable.
package main

import "fmt"

type cell struct{ r, c, steps int }

func shortestPath(board [][]bool, start, end [2]int) int {
	m, n := len(board), len(board[0])
	if board[start[0]][start[1]] || board[end[0]][end[1]] {
		return -1
	}
	seen := make([][]bool, m)
	for i := range seen {
		seen[i] = make([]bool, n)
	}
	q := []cell{{start[0], start[1], 0}}
	seen[start[0]][start[1]] = true
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		if cur.r == end[0] && cur.c == end[1] {
			return cur.steps
		}
		for _, d := range dirs {
			nr, nc := cur.r+d[0], cur.c+d[1]
			if nr < 0 || nr >= m || nc < 0 || nc >= n {
				continue
			}
			if seen[nr][nc] || board[nr][nc] {
				continue
			}
			seen[nr][nc] = true
			q = append(q, cell{nr, nc, cur.steps + 1})
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
