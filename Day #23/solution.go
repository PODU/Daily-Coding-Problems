// Shortest path on a grid with walls using BFS (4-directional).
// Time: O(M*N), Space: O(M*N).
package main

import "fmt"

func shortestPath(board [][]bool, start, end [2]int) (int, bool) {
	m, n := len(board), len(board[0])
	visited := make([][]bool, m)
	for i := range visited {
		visited[i] = make([]bool, n)
	}
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	type node struct{ r, c, d int }
	q := []node{{start[0], start[1], 0}}
	visited[start[0]][start[1]] = true
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		if cur.r == end[0] && cur.c == end[1] {
			return cur.d, true
		}
		for _, dd := range dirs {
			nr, nc := cur.r+dd[0], cur.c+dd[1]
			if nr >= 0 && nr < m && nc >= 0 && nc < n && !visited[nr][nc] && !board[nr][nc] {
				visited[nr][nc] = true
				q = append(q, node{nr, nc, cur.d + 1})
			}
		}
	}
	return 0, false
}

func main() {
	f, t := false, true
	board := [][]bool{
		{f, f, f, f},
		{t, t, f, t},
		{f, f, f, f},
		{f, f, f, f},
	}
	res, ok := shortestPath(board, [2]int{3, 0}, [2]int{0, 0})
	if !ok {
		fmt.Println("None")
	} else {
		fmt.Println(res)
	}
}
