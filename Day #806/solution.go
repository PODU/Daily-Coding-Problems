// Day 806: In a row/col-sorted matrix, count elements < M[i1][j1] plus those
// >= M[i2][j2] (boundary inclusive on high side to match the sample's 15).
// Per row binary search. Time O(N log M), Space O(1).
package main

import (
	"fmt"
	"sort"
)

func countOutside(M [][]int, i1, j1, i2, j2 int) int {
	low, high := M[i1][j1], M[i2][j2]
	total := 0
	for _, row := range M {
		total += sort.SearchInts(row, low)             // elements < low
		total += len(row) - sort.SearchInts(row, high) // elements >= high
	}
	return total
}

func main() {
	M := [][]int{
		{1, 3, 7, 10, 15, 20}, {2, 6, 9, 14, 22, 25}, {3, 8, 10, 15, 25, 30},
		{10, 11, 12, 23, 30, 35}, {20, 25, 30, 35, 40, 45},
	}
	fmt.Println(countOutside(M, 1, 1, 3, 3)) // 15
}
