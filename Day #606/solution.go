// Count all open knight's tours: DFS backtracking from every start square,
// counting Hamiltonian paths. Time O(8^(N*N)) worst, Space O(N*N). N=5 -> 1728.
package main

import "fmt"

var dx = []int{1, 1, -1, -1, 2, 2, -2, -2}
var dy = []int{2, -2, 2, -2, 1, -1, 1, -1}

func dfs(x, y, visited, n int, board [][]bool) int64 {
	if visited == n*n {
		return 1
	}
	var count int64 = 0
	for k := 0; k < 8; k++ {
		nx, ny := x+dx[k], y+dy[k]
		if nx >= 0 && nx < n && ny >= 0 && ny < n && !board[nx][ny] {
			board[nx][ny] = true
			count += dfs(nx, ny, visited+1, n, board)
			board[nx][ny] = false
		}
	}
	return count
}

func knightTours(n int) int64 {
	if n == 0 {
		return 0
	}
	var total int64 = 0
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			board := make([][]bool, n)
			for r := range board {
				board[r] = make([]bool, n)
			}
			board[i][j] = true
			total += dfs(i, j, 1, n, board)
		}
	}
	return total
}

func main() {
	fmt.Println(knightTours(5))
}
