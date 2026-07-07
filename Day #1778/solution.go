// Largest sum of non-adjacent numbers via include/exclude DP; empty selection (0) allowed.
// Time: O(N), Space: O(1).
package main

import "fmt"

func maxNonAdjacent(a []int) int64 {
	var incl, excl int64 = 0, 0
	for _, n := range a {
		ni := excl + int64(n)
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
	fmt.Println(maxNonAdjacent([]int{2, 4, 6, 2, 5}))
	fmt.Println(maxNonAdjacent([]int{5, 1, 1, 5}))
}
