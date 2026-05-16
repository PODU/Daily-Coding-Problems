// Count cells equal to X in an N x N multiplication table (cell(i,j)=i*j).
// For each row i, X is at column X/i iff i divides X and 1<=X/i<=N. O(N) time, O(1) space.
package main

import "fmt"

func countCells(n, x int64) int {
	count := 0
	for i := int64(1); i <= n; i++ {
		if x%i == 0 {
			j := x / i
			if j >= 1 && j <= n {
				count++
			}
		}
	}
	return count
}

func main() {
	fmt.Println(countCells(6, 12)) // expected 4
}
