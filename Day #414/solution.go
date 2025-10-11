// Day 414: Count N-Queens arrangements via bitmask backtracking.
// Track used columns/diagonals as bitmasks. Time O(N!)-ish, Space O(N).
package main

import "fmt"

func solve(n, row, cols, diag1, diag2 int) int {
	if row == n {
		return 1
	}
	count := 0
	avail := ((1 << n) - 1) & ^(cols | diag1 | diag2)
	for avail != 0 {
		bit := avail & (-avail)
		avail -= bit
		count += solve(n, row+1, cols|bit, (diag1|bit)<<1, (diag2|bit)>>1)
	}
	return count
}

func countNQueens(n int) int { return solve(n, 0, 0, 0, 0) }

func main() {
	for n := 1; n <= 8; n++ {
		fmt.Printf("N=%d: %d\n", n, countNQueens(n))
	}
	fmt.Println(countNQueens(8)) // 92
}
