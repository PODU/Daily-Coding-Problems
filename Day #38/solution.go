// N-Queens count via bitmask backtracking (columns + two diagonals). O(N!) worst, O(N) space.
package main

import "fmt"

func solve(cols, diag1, diag2, full int) int {
	if cols == full {
		return 1
	}
	count := 0
	avail := ^(cols | diag1 | diag2) & full
	for avail != 0 {
		bit := avail & (-avail)
		avail -= bit
		count += solve(cols|bit, (diag1|bit)<<1, (diag2|bit)>>1, full)
	}
	return count
}

func countNQueens(n int) int {
	return solve(0, 0, 0, (1<<n)-1)
}

func main() {
	fmt.Println(countNQueens(8))
}
