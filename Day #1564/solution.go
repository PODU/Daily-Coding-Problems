// Staircase count from top-right in O(N+M): smaller = count(<low); larger = N*M - count(<high). Time O(N+M), Space O(1).
package main

import "fmt"

// countLess returns the number of elements strictly less than x in a row/col sorted matrix.
func countLess(M [][]int, x int) int {
	n, m := len(M), len(M[0])
	cnt := 0
	r, c := 0, m-1
	for r < n && c >= 0 {
		if M[r][c] < x {
			cnt += c + 1
			r++
		} else {
			c--
		}
	}
	return cnt
}

func main() {
	M := [][]int{
		{1, 3, 7, 10, 15, 20},
		{2, 6, 9, 14, 22, 25},
		{3, 8, 10, 15, 25, 30},
		{10, 11, 12, 23, 30, 35},
		{20, 25, 30, 35, 40, 45},
	}
	low := M[1][1]  // 6
	high := M[3][3] // 23
	total := len(M) * len(M[0])
	smaller := countLess(M, low)         // elements < 6
	larger := total - countLess(M, high) // elements >= 23
	fmt.Println(smaller + larger)
}
