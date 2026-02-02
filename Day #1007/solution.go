// King-in-check: locate K, test rook/queen lines, bishop/queen diagonals, knight, king, pawn.
// Row 0 is top; white pawns attack upward (toward smaller row). Time O(64), Space O(1).
package main

import "fmt"

func inBoard(r, c int) bool { return r >= 0 && r < 8 && c >= 0 && c < 8 }

func isCheck(b []string) bool {
	kr, kc := -1, -1
	for r := 0; r < 8; r++ {
		for c := 0; c < 8; c++ {
			if b[r][c] == 'K' {
				kr, kc = r, c
			}
		}
	}

	for _, d := range [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}} {
		r, c := kr+d[0], kc+d[1]
		for inBoard(r, c) {
			p := b[r][c]
			if p != '.' {
				if p == 'R' || p == 'Q' {
					return true
				}
				break
			}
			r += d[0]
			c += d[1]
		}
	}
	for _, d := range [][2]int{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}} {
		r, c := kr+d[0], kc+d[1]
		for inBoard(r, c) {
			p := b[r][c]
			if p != '.' {
				if p == 'B' || p == 'Q' {
					return true
				}
				break
			}
			r += d[0]
			c += d[1]
		}
	}
	for _, d := range [][2]int{{1, 2}, {1, -2}, {-1, 2}, {-1, -2}, {2, 1}, {2, -1}, {-2, 1}, {-2, -1}} {
		r, c := kr+d[0], kc+d[1]
		if inBoard(r, c) && b[r][c] == 'N' {
			return true
		}
	}
	for dr := -1; dr <= 1; dr++ {
		for dc := -1; dc <= 1; dc++ {
			if dr == 0 && dc == 0 {
				continue
			}
			r, c := kr+dr, kc+dc
			if inBoard(r, c) && b[r][c] == 'K' {
				return true
			}
		}
	}
	for _, dc := range []int{-1, 1} {
		r, c := kr+1, kc+dc
		if inBoard(r, c) && b[r][c] == 'P' {
			return true
		}
	}
	return false
}

func main() {
	board := []string{
		"...K....",
		"........",
		".B......",
		"......P.",
		".......R",
		"..N.....",
		"........",
		".....Q..",
	}
	if isCheck(board) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
