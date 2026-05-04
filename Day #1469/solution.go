// Chess check detection: locate black king, test pawn/knight attacks and ray-cast
// bishop/rook/queen lines blocked by pieces. Time: O(64); Space: O(1) extra.
package main

import "fmt"

func inCheck(b []string) bool {
	kr, kc := -1, -1
	for r := 0; r < 8; r++ {
		for c := 0; c < 8; c++ {
			if b[r][c] == 'K' {
				kr, kc = r, c
			}
		}
	}
	if kr < 0 {
		return false
	}

	// White pawns move up; a pawn at (kr+1, kc+-1) attacks the king.
	for _, dc := range []int{-1, 1} {
		pr, pc := kr+1, kc+dc
		if pr >= 0 && pr < 8 && pc >= 0 && pc < 8 && b[pr][pc] == 'P' {
			return true
		}
	}

	km := [][2]int{{1, 2}, {1, -2}, {-1, 2}, {-1, -2}, {2, 1}, {2, -1}, {-2, 1}, {-2, -1}}
	for _, m := range km {
		r, c := kr+m[0], kc+m[1]
		if r >= 0 && r < 8 && c >= 0 && c < 8 && b[r][c] == 'N' {
			return true
		}
	}

	diag := [][2]int{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}}
	for _, d := range diag {
		r, c := kr+d[0], kc+d[1]
		for r >= 0 && r < 8 && c >= 0 && c < 8 {
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

	strt := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for _, d := range strt {
		r, c := kr+d[0], kc+d[1]
		for r >= 0 && r < 8 && c >= 0 && c < 8 {
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
	if inCheck(board) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
