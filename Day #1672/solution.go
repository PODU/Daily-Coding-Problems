// Day 1672: Determine if black king is in check on an 8x8 board.
// Scan knight jumps + 8 rays from king (first blocker decides). Time O(64), Space O(1).
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
	km := [8][2]int{{-2, -1}, {-2, 1}, {-1, -2}, {-1, 2}, {1, -2}, {1, 2}, {2, -1}, {2, 1}}
	for _, m := range km {
		r, c := kr+m[0], kc+m[1]
		if r >= 0 && r < 8 && c >= 0 && c < 8 && b[r][c] == 'N' {
			return true
		}
	}
	dirs := [8][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}, {1, 1}, {1, -1}, {-1, 1}, {-1, -1}}
	for d := 0; d < 8; d++ {
		diag := d >= 4
		for step := 1; step < 8; step++ {
			r, c := kr+dirs[d][0]*step, kc+dirs[d][1]*step
			if r < 0 || r >= 8 || c < 0 || c >= 8 {
				break
			}
			p := b[r][c]
			if p == '.' {
				continue
			}
			if diag {
				if p == 'B' || p == 'Q' {
					return true
				}
				if step == 1 && p == 'K' {
					return true
				}
				if step == 1 && p == 'P' && dirs[d][0] == 1 {
					return true
				}
			} else {
				if p == 'R' || p == 'Q' {
					return true
				}
				if step == 1 && p == 'K' {
					return true
				}
			}
			break
		}
	}
	return false
}

func main() {
	board := []string{
		"...K....", "........", ".B......", "......P.",
		".......R", "..N.....", "........", ".....Q.."}
	if inCheck(board) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
