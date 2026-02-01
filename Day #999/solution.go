// Day 999: Count occurrences of X in an N x N multiplication table.
// X appears at (i, j) iff i divides X and X/i <= N, for i in 1..N. O(N) time.
package main

import "fmt"

func countX(n, x int) int {
	cnt := 0
	for i := 1; i <= n; i++ {
		if x%i == 0 && x/i <= n {
			cnt++
		}
	}
	return cnt
}

func main() {
	fmt.Println(countX(6, 12)) // 4
}
