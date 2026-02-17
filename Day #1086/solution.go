// Sudoku solver via backtracking with bitmask constraints (rows/cols/boxes).
// Worst-case exponential, fast in practice; Space O(1).
package main

import "fmt"

var rowM, colM, boxM [9]int
var grid [9][9]int

func boxId(r, c int) int { return (r/3)*3 + c/3 }

func solve(pos int) bool {
	if pos == 81 {
		return true
	}
	r, c := pos/9, pos%9
	if grid[r][c] != 0 {
		return solve(pos + 1)
	}
	b := boxId(r, c)
	for d := 1; d <= 9; d++ {
		bit := 1 << uint(d)
		if (rowM[r]|colM[c]|boxM[b])&bit == 0 {
			grid[r][c] = d
			rowM[r] |= bit
			colM[c] |= bit
			boxM[b] |= bit
			if solve(pos + 1) {
				return true
			}
			grid[r][c] = 0
			rowM[r] &^= bit
			colM[c] &^= bit
			boxM[b] &^= bit
		}
	}
	return false
}

func main() {
	puzzle := [9][9]int{
		{5, 3, 0, 0, 7, 0, 0, 0, 0},
		{6, 0, 0, 1, 9, 5, 0, 0, 0},
		{0, 9, 8, 0, 0, 0, 0, 6, 0},
		{8, 0, 0, 0, 6, 0, 0, 0, 3},
		{4, 0, 0, 8, 0, 3, 0, 0, 1},
		{7, 0, 0, 0, 2, 0, 0, 0, 6},
		{0, 6, 0, 0, 0, 0, 2, 8, 0},
		{0, 0, 0, 4, 1, 9, 0, 0, 5},
		{0, 0, 0, 0, 8, 0, 0, 7, 9}}
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			grid[r][c] = puzzle[r][c]
			if puzzle[r][c] != 0 {
				bit := 1 << uint(puzzle[r][c])
				rowM[r] |= bit
				colM[c] |= bit
				boxM[boxId(r, c)] |= bit
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
