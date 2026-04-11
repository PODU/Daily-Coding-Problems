// Day 1336: Count distinct N-Queens arrangements.
// Backtracking with column/diagonal bitmasks. Time: O(N!) worst, Space: O(N).
package main

import "fmt"

func count(n, row, cols, d1, d2 int) int {
	if row == n {
		return 1
	}
	total := 0
	avail := ((1 << n) - 1) &^ (cols | d1 | d2)
	for avail != 0 {
		bit := avail & (-avail)
		avail -= bit
		total += count(n, row+1, cols|bit, (d1|bit)<<1, (d2|bit)>>1)
	}
	return total
}

func totalNQueens(n int) int { return count(n, 0, 0, 0, 0) }

func main() {
	fmt.Println("N=1 ->", totalNQueens(1))
	fmt.Println("N=4 ->", totalNQueens(4))
	fmt.Println("N=8 ->", totalNQueens(8))
}
