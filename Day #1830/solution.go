// Word search: DFS backtracking from each cell. O(R*C*4^L) time, O(L) space.
package main

import "fmt"

func dfs(b [][]byte, w string, i, r, c int) bool {
	if i == len(w) {
		return true
	}
	if r < 0 || r >= len(b) || c < 0 || c >= len(b[0]) || b[r][c] != w[i] {
		return false
	}
	saved := b[r][c]
	b[r][c] = '#'
	found := dfs(b, w, i+1, r+1, c) || dfs(b, w, i+1, r-1, c) ||
		dfs(b, w, i+1, r, c+1) || dfs(b, w, i+1, r, c-1)
	b[r][c] = saved
	return found
}

func exists(b [][]byte, w string) bool {
	for r := 0; r < len(b); r++ {
		for c := 0; c < len(b[0]); c++ {
			if dfs(b, w, 0, r, c) {
				return true
			}
		}
	}
	return false
}

func main() {
	board := [][]byte{
		[]byte("ABCE"),
		[]byte("SFCS"),
		[]byte("ADEE"),
	}
	for _, w := range []string{"ABCCED", "SEE", "ABCB"} {
		res := "false"
		if exists(board, w) {
			res = "true"
		}
		fmt.Printf("exists(board, \"%s\") returns %s\n", w, res)
	}
}
