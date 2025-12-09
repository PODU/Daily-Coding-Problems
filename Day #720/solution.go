// Day 720: Sudoku solver via backtracking with bitmasks for rows/cols/boxes,
// always filling the next empty cell. Time exponential worst-case but fast in practice.
package main

import "fmt"

var (
	rows, cols, boxes [9]int
	grid              [9][9]int
)

func boxIdx(r, c int) int { return (r/3)*3 + c/3 }

func solve(pos int) bool {
	if pos == 81 {
		return true
	}
	r, c := pos/9, pos%9
	if grid[r][c] != 0 {
		return solve(pos + 1)
	}
	b := boxIdx(r, c)
	for d := 1; d <= 9; d++ {
		bit := 1 << uint(d)
		if (rows[r]|cols[c]|boxes[b])&bit != 0 {
			continue
		}
		grid[r][c] = d
		rows[r] |= bit
		cols[c] |= bit
		boxes[b] |= bit
		if solve(pos + 1) {
			return true
		}
		grid[r][c] = 0
		rows[r] &^= bit
		cols[c] &^= bit
		boxes[b] &^= bit
	}
	return false
}

func main() {
	puzzle := []string{
		"530070000", "600195000", "098000060",
		"800060003", "400803001", "700020006",
		"060000280", "000419005", "000080079"}
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			grid[r][c] = int(puzzle[r][c] - '0')
			if grid[r][c] != 0 {
				bit := 1 << uint(grid[r][c])
				rows[r] |= bit
				cols[c] |= bit
				boxes[boxIdx(r, c)] |= bit
			}
		}
	}
	solve(0)
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			fmt.Print(grid[r][c])
		}
		fmt.Println()
	}
}
