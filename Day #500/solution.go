// BFS shortest path on a boolean grid (false=walkable, true=wall).
// Time O(M*N), Space O(M*N).
package main

import "fmt"

func minSteps(board [][]bool, start, end [2]int) int {
	M, N := len(board), len(board[0])
	if board[start[0]][start[1]] || board[end[0]][end[1]] {
		return -1
	}
	visited := make([][]bool, M)
	for i := range visited {
		visited[i] = make([]bool, N)
	}
	q := [][2]int{start}
	visited[start[0]][start[1]] = true
	steps := 0
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for len(q) > 0 {
		next := [][2]int{}
		for _, cur := range q {
			if cur == end {
				return steps
			}
			for _, d := range dirs {
				nr, nc := cur[0]+d[0], cur[1]+d[1]
				if nr >= 0 && nr < M && nc >= 0 && nc < N && !visited[nr][nc] && !board[nr][nc] {
					visited[nr][nc] = true
					next = append(next, [2]int{nr, nc})
				}
			}
		}
		q = next
		steps++
	}
	return -1
}

func main() {
	t, f := true, false
	board := [][]bool{
		{f, f, f, f},
		{t, t, f, t},
		{f, f, f, f},
		{f, f, f, f},
	}
	fmt.Println(minSteps(board, [2]int{3, 0}, [2]int{0, 0}))
}
