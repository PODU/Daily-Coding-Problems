// Word Search: DFS backtracking from each cell, mark visited then restore.
// Time O(M*N*4^L), Space O(L) recursion depth.
package main

import "fmt"

func dfs(b [][]byte, w string, i, j, k int) bool {
	if k == len(w) {
		return true
	}
	if i < 0 || j < 0 || i >= len(b) || j >= len(b[0]) || b[i][j] != w[k] {
		return false
	}
	tmp := b[i][j]
	b[i][j] = '#'
	found := dfs(b, w, i+1, j, k+1) || dfs(b, w, i-1, j, k+1) ||
		dfs(b, w, i, j+1, k+1) || dfs(b, w, i, j-1, k+1)
	b[i][j] = tmp
	return found
}

func exists(board [][]byte, word string) bool {
	for i := range board {
		for j := range board[0] {
			if dfs(board, word, i, j, 0) {
				return true
			}
		}
	}
	return false
}

func clone(b [][]byte) [][]byte {
	c := make([][]byte, len(b))
	for i := range b {
		c[i] = append([]byte(nil), b[i]...)
	}
	return c
}

func main() {
	board := [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}}
	fmt.Println(exists(clone(board), "ABCCED"))
	fmt.Println(exists(clone(board), "SEE"))
	fmt.Println(exists(clone(board), "ABCB"))
}
