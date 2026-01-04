// Day 855: count N-Queens solutions via backtracking with bitmasks for column/diagonals.
// O(N!) worst case, O(N) space. Bitmask makes conflict checks O(1).
package main

import "fmt"

func countNQueens(n int) int64 {
	full := (1 << n) - 1
	var count int64
	var solve func(cols, diag1, diag2 int)
	solve = func(cols, diag1, diag2 int) {
		if cols == full {
			count++
			return
		}
		avail := full & ^(cols | diag1 | diag2)
		for avail != 0 {
			p := avail & (-avail)
			avail -= p
			solve(cols|p, (diag1|p)<<1, (diag2|p)>>1)
		}
	}
	solve(0, 0, 0)
	return count
}

func main() {
	for n := 1; n <= 8; n++ {
		fmt.Printf("N=%d: %d\n", n, countNQueens(n)) // N=8: 92
	}
}
