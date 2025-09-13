// Day 267: Is the black king in check? Locate K, then cast rook/queen orthogonal
// rays and bishop/queen diagonal rays (stopping at the first blocker), and test
// knight and pawn attacks. Time O(8x8) = O(1); space O(1).
package main

import "fmt"

func inCheck(board []string) bool {
	kr, kc := -1, -1
	for r := 0; r < 8; r++ {
		for c := 0; c < 8; c++ {
			if board[r][c] == 'K' {
				kr, kc = r, c
			}
		}
	}
	if kr < 0 {
		return false
	}
	at := func(r, c int) byte {
		if r < 0 || r >= 8 || c < 0 || c >= 8 {
			return 0
		}
		return board[r][c]
	}

	orth := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for _, d := range orth {
		r, c := kr+d[0], kc+d[1]
		for at(r, c) == '.' {
			r += d[0]
			c += d[1]
		}
		if p := at(r, c); p == 'R' || p == 'Q' {
			return true
		}
	}
	diag := [4][2]int{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}}
	for _, d := range diag {
		r, c := kr+d[0], kc+d[1]
		for at(r, c) == '.' {
			r += d[0]
			c += d[1]
		}
		if p := at(r, c); p == 'B' || p == 'Q' {
			return true
		}
	}
	kn := [8][2]int{{1, 2}, {1, -2}, {-1, 2}, {-1, -2}, {2, 1}, {2, -1}, {-2, 1}, {-2, -1}}
	for _, d := range kn {
		if at(kr+d[0], kc+d[1]) == 'N' {
			return true
		}
	}
	if at(kr+1, kc-1) == 'P' || at(kr+1, kc+1) == 'P' {
		return true
	}
	for dr := -1; dr <= 1; dr++ {
		for dc := -1; dc <= 1; dc++ {
			if (dr != 0 || dc != 0) && at(kr+dr, kc+dc) == 'k' {
				return true
			}
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
