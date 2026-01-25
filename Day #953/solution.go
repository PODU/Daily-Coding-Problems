// Day 953: largest sum of non-adjacent numbers (may pick none -> >= 0).
// incl/excl DP. Time O(n), Space O(1).
package main

import "fmt"

func maxNonAdjacent(a []int64) int64 {
	var incl, excl int64 = 0, 0
	for _, x := range a {
		ni := excl + x
		ne := incl
		if excl > ne {
			ne = excl
		}
		incl, excl = ni, ne
	}
	if incl > excl {
		return incl
	}
	return excl
}

func main() {
	fmt.Println(maxNonAdjacent([]int64{2, 4, 6, 2, 5})) // 13
	fmt.Println(maxNonAdjacent([]int64{5, 1, 1, 5}))    // 10
}
