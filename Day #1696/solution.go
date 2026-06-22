// N-Queens count via backtracking with bitmasks (columns + two diagonals).
// Time O(N!) worst case (heavily pruned), Space O(N) recursion.
package main

import "fmt"

func solve(n, row, cols, diag1, diag2 int) int {
	if row == n {
		return 1
	}
	count := 0
	avail := ((1 << uint(n)) - 1) &^ (cols | diag1 | diag2)
	for avail != 0 {
		bit := avail & (-avail)
		avail -= bit
		count += solve(n, row+1, cols|bit, (diag1|bit)<<1, (diag2|bit)>>1)
	}
	return count
}

func totalNQueens(n int) int {
	return solve(n, 0, 0, 0, 0)
}

func main() {
	fmt.Println(totalNQueens(8)) // 92
}
