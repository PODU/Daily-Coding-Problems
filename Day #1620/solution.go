// Sudoku solver: backtracking with row/col/box bitmasks; pick first empty cell.
// Time: exponential worst case, fast in practice. Space: O(1) extra (fixed 9x9).
package main

import "fmt"

var rows, cols, boxes [9]int
var grid [9][9]byte

func boxIndex(r, c int) int { return (r/3)*3 + c/3 }

func solve(pos int) bool {
	if pos == 81 {
		return true
	}
	r, c := pos/9, pos%9
	if grid[r][c] != '0' && grid[r][c] != '.' {
		return solve(pos + 1)
	}
	b := boxIndex(r, c)
	for d := 1; d <= 9; d++ {
		bit := 1 << uint(d)
		if rows[r]&bit != 0 || cols[c]&bit != 0 || boxes[b]&bit != 0 {
			continue
		}
		rows[r] |= bit
		cols[c] |= bit
		boxes[b] |= bit
		grid[r][c] = byte('0' + d)
		if solve(pos + 1) {
			return true
		}
		rows[r] &^= bit
		cols[c] &^= bit
		boxes[b] &^= bit
		grid[r][c] = '.'
	}
	return false
}

func main() {
	puzzle := []string{
		"53..7....",
		"6..195...",
		".98....6.",
		"8...6...3",
		"4..8.3..1",
		"7...2...6",
		".6....28.",
		"...419..5",
		"....8..79",
	}
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			ch := puzzle[r][c]
			grid[r][c] = ch
			if ch != '.' && ch != '0' {
				bit := 1 << uint(ch-'0')
				rows[r] |= bit
				cols[c] |= bit
				boxes[boxIndex(r, c)] |= bit
			}
		}
	}
	solve(0)
	for r := 0; r < 9; r++ {
		fmt.Println(string(grid[r][:]))
	}
}
