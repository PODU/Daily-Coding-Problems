// Word search L-to-R / U-to-D only: scan each row and column for target substring.
// Time O(R*C*L), Space O(max(R,C)).
package main

import (
	"fmt"
	"strings"
)

func findWord(m [][]byte, target string) bool {
	R := len(m)
	C := 0
	if R > 0 {
		C = len(m[0])
	}
	for r := 0; r < R; r++ {
		if strings.Contains(string(m[r]), target) {
			return true
		}
	}
	for c := 0; c < C; c++ {
		var col strings.Builder
		for r := 0; r < R; r++ {
			col.WriteByte(m[r][c])
		}
		if strings.Contains(col.String(), target) {
			return true
		}
	}
	return false
}

func main() {
	matrix := [][]byte{
		{'F', 'A', 'C', 'I'},
		{'O', 'B', 'Q', 'P'},
		{'A', 'N', 'O', 'B'},
		{'M', 'A', 'S', 'S'},
	}
	fmt.Println(findWord(matrix, "FOAM"))
}
