// Day 425: Chess check detection. Knight/pawn offsets + sliding rays (rook/bishop/queen)
// with blocking. O(1) board scan from the king's square.
package main

import "fmt"

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
	n := 8
	kr, kc := -1, -1
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if board[i][j] == 'K' {
				kr, kc = i, j
			}
		}
	}
	inb := func(r, c int) bool { return r >= 0 && r < n && c >= 0 && c < n }
	check := false

	knight := [][2]int{{-2, -1}, {-2, 1}, {-1, -2}, {-1, 2}, {1, -2}, {1, 2}, {2, -1}, {2, 1}}
	for _, d := range knight {
		r, c := kr+d[0], kc+d[1]
		if inb(r, c) && board[r][c] == 'N' {
			check = true
		}
	}

	for _, dc := range []int{-1, 1} {
		r, c := kr+1, kc+dc
		if inb(r, c) && board[r][c] == 'P' {
			check = true
		}
	}

	orth := [][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for _, d := range orth {
		r, c := kr+d[0], kc+d[1]
		for inb(r, c) {
			p := board[r][c]
			if p != '.' {
				if p == 'R' || p == 'Q' {
					check = true
				}
				break
			}
			r += d[0]
			c += d[1]
		}
	}

	diag := [][2]int{{-1, -1}, {-1, 1}, {1, -1}, {1, 1}}
	for _, d := range diag {
		r, c := kr+d[0], kc+d[1]
		for inb(r, c) {
			p := board[r][c]
			if p != '.' {
				if p == 'B' || p == 'Q' {
					check = true
				}
				break
			}
			r += d[0]
			c += d[1]
		}
	}

	if check {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
