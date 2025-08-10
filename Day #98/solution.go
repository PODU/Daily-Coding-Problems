// Day 98: Word search via DFS backtracking from each cell, marking visited cells
// in place. O(M*N*4^L) time, O(L) recursion space.
package main

import "fmt"

func dfs(b [][]byte, w string, r, c, i int) bool {
	if i == len(w) {
		return true
	}
	if r < 0 || r >= len(b) || c < 0 || c >= len(b[0]) || b[r][c] != w[i] {
		return false
	}
	saved := b[r][c]
	b[r][c] = '#'
	found := dfs(b, w, r+1, c, i+1) || dfs(b, w, r-1, c, i+1) ||
		dfs(b, w, r, c+1, i+1) || dfs(b, w, r, c-1, i+1)
	b[r][c] = saved
	return found
}

func exists(board [][]byte, w string) bool {
	for r := range board {
		for c := range board[0] {
			if dfs(board, w, r, c, 0) {
				return true
			}
		}
	}
	return false
}

func main() {
	board := [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}}
	fmt.Println(exists(board, "ABCCED")) // true
	fmt.Println(exists(board, "SEE"))    // true
	fmt.Println(exists(board, "ABCB"))   // false
}
