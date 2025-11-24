// Per-row binary search: count elements < A[i1][j1] (lowerBound) plus elements >= A[i2][j2] (M - lowerBound).
// Upper bound taken inclusive (>=) to match reference example. Time O(N log M), space O(1).
package main

import (
	"fmt"
	"sort"
)

func main() {
	A := [][]int{
		{1, 3, 7, 10, 15, 20},
		{2, 6, 9, 14, 22, 25},
		{3, 8, 10, 15, 25, 30},
		{10, 11, 12, 23, 30, 35},
		{20, 25, 30, 35, 40, 45},
	}
	i1, j1, i2, j2 := 1, 1, 3, 3
	pivot1 := A[i1][j1] // 6
	pivot2 := A[i2][j2] // 23
	M := len(A[0])
	countSmaller, countUpper := 0, 0
	for _, row := range A {
		countSmaller += sort.SearchInts(row, pivot1)     // strictly less than pivot1
		countUpper += M - sort.SearchInts(row, pivot2)   // >= pivot2 (inclusive)
	}
	fmt.Println(countSmaller + countUpper)
}
