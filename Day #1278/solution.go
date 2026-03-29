// Day 1278: Sudoku solver via backtracking with row/col/box bitmasks.
// Time: exponential worst-case but fast with constraint pruning. Space O(1).
package main

import "fmt"

var g [9][9]int
var rowm, colm, boxm [9]int

func bidx(r, c int) int { return (r/3)*3 + c/3 }

func solve(pos int) bool {
	if pos == 81 {
		return true
	}
	r, c := pos/9, pos%9
	if g[r][c] != 0 {
		return solve(pos + 1)
	}
	b := bidx(r, c)
	used := rowm[r] | colm[c] | boxm[b]
	for d := 1; d <= 9; d++ {
		bit := 1 << d
		if used&bit != 0 {
			continue
		}
		g[r][c] = d
		rowm[r] |= bit
		colm[c] |= bit
		boxm[b] |= bit
		if solve(pos + 1) {
			return true
		}
		g[r][c] = 0
		rowm[r] &^= bit
		colm[c] &^= bit
		boxm[b] &^= bit
	}
	return false
}

func main() {
	puzzle := []string{
		"53..7....", "6..195...", ".98....6.",
		"8...6...3", "4..8.3..1", "7...2...6",
		".6....28.", "...419..5", "....8..79"}
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			ch := puzzle[r][c]
			v := 0
			if ch != '.' {
				v = int(ch - '0')
			}
			g[r][c] = v
			if v != 0 {
				bit := 1 << v
				rowm[r] |= bit
				colm[c] |= bit
				boxm[bidx(r, c)] |= bit
			}
		}
	}
	solve(0)
	for r := 0; r < 9; r++ {
		line := ""
		for c := 0; c < 9; c++ {
			line += fmt.Sprintf("%d", g[r][c])
		}
		fmt.Println(line)
	}
}
