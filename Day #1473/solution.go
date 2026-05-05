// Day 1473: Count occurrences of X in an N x N multiplication table.
// For each row i, X appears iff i divides X and X/i is within [1, N].
// Time O(N), Space O(1).
package main

import "fmt"

func countX(n, x int64) int64 {
	var count int64
	for i := int64(1); i <= n; i++ {
		if x%i == 0 && x/i >= 1 && x/i <= n {
			count++
		}
	}
	return count
}

func main() {
	fmt.Println(countX(6, 12)) // 4
}
