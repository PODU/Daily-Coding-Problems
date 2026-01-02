// Day 840: Print a string in zigzag form across k lines.
// Char i sits at column i; its row follows the zigzag 0,1,..,k-1,k-2,..,1,0,...
// Build k rows of spaces, place each char, print with trailing spaces trimmed. Time O(N*k).
package main

import (
	"fmt"
	"strings"
)

func zigzag(s string, k int) string {
	if k <= 0 {
		return ""
	}
	if k == 1 {
		return s
	}
	n := len(s)
	rows := make([][]byte, k)
	for i := range rows {
		rows[i] = make([]byte, n)
		for j := range rows[i] {
			rows[i][j] = ' '
		}
	}
	row, step := 0, 1
	for i := 0; i < n; i++ {
		rows[row][i] = s[i]
		if row == 0 {
			step = 1
		} else if row == k-1 {
			step = -1
		}
		row += step
	}
	lines := make([]string, k)
	for i, r := range rows {
		lines[i] = strings.TrimRight(string(r), " ")
	}
	return strings.Join(lines, "\n")
}

func main() {
	fmt.Println(zigzag("thisisazigzag", 4))
}
