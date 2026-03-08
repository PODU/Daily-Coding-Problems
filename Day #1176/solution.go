// Day 1176: Word search in a 2D board via DFS backtracking.
// Try each cell as a start, explore 4 neighbors, mark visited in-place.
// Time O(M*N*4^L), Space O(L) recursion (L = word length).
package main

import "fmt"

func exists(board [][]byte, word string) bool {
	rows, cols := len(board), len(board[0])
	var dfs func(k, i, j int) bool
	dfs = func(k, i, j int) bool {
		if k == len(word) {
			return true
		}
		if i < 0 || i >= rows || j < 0 || j >= cols || board[i][j] != word[k] {
			return false
		}
		saved := board[i][j]
		board[i][j] = '#'
		found := dfs(k+1, i+1, j) || dfs(k+1, i-1, j) ||
			dfs(k+1, i, j+1) || dfs(k+1, i, j-1)
		board[i][j] = saved
		return found
	}
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if dfs(0, i, j) {
				return true
			}
		}
	}
	return false
}

func newBoard() [][]byte {
	return [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}}
}

func main() {
	fmt.Println(exists(newBoard(), "ABCCED"))
	fmt.Println(exists(newBoard(), "SEE"))
	fmt.Println(exists(newBoard(), "ABCB"))
}
