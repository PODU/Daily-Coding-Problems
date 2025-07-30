// Day 54: Sudoku solver via backtracking with bitmask row/col/box constraints.
// Worst case exponential; bitmasks make pruning fast. Space: O(1).
package main

import "fmt"

var rows, cols, boxes [9]int

func solve(g *[9][9]int, cell int) bool {
	if cell == 81 {
		return true
	}
	r, c := cell/9, cell%9
	b := (r/3)*3 + c/3
	if g[r][c] != 0 {
		return solve(g, cell+1)
	}
	for d := 1; d <= 9; d++ {
		bit := 1 << uint(d)
		if (rows[r]|cols[c]|boxes[b])&bit != 0 {
			continue
		}
		g[r][c] = d
		rows[r] |= bit
		cols[c] |= bit
		boxes[b] |= bit
		if solve(g, cell+1) {
			return true
		}
		g[r][c] = 0
		rows[r] &^= bit
		cols[c] &^= bit
		boxes[b] &^= bit
	}
	return false
}

func main() {
	g := [9][9]int{
		{5, 3, 0, 0, 7, 0, 0, 0, 0},
		{6, 0, 0, 1, 9, 5, 0, 0, 0},
		{0, 9, 8, 0, 0, 0, 0, 6, 0},
		{8, 0, 0, 0, 6, 0, 0, 0, 3},
		{4, 0, 0, 8, 0, 3, 0, 0, 1},
		{7, 0, 0, 0, 2, 0, 0, 0, 6},
		{0, 6, 0, 0, 0, 0, 2, 8, 0},
		{0, 0, 0, 4, 1, 9, 0, 0, 5},
		{0, 0, 0, 0, 8, 0, 0, 7, 9},
	}
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			if g[r][c] != 0 {
				bit := 1 << uint(g[r][c])
				rows[r] |= bit
				cols[c] |= bit
				boxes[(r/3)*3+c/3] |= bit
			}
		}
	}
	solve(&g, 0)
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			fmt.Print(g[r][c])
		}
		fmt.Println()
	}
}
