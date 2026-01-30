// Day 987: Nearest larger number index.
// Expand outward from i checking i-d and i+d; first larger wins. O(n) per query, O(1) space.
// Follow-up: with O(n^2) preprocessing (or sparse tables) each query can be answered in O(1).
package main

import "fmt"

// nearestLarger returns (index, true) of nearest larger element, or (-1, false) if none.
func nearestLarger(a []int, i int) (int, bool) {
	n := len(a)
	for d := 1; d < n; d++ {
		l, r := i-d, i+d
		if l >= 0 && a[l] > a[i] {
			return l, true // prefer left on ties
		}
		if r < n && a[r] > a[i] {
			return r, true
		}
	}
	return -1, false
}

func main() {
	a := []int{4, 1, 3, 5, 6}
	idx, ok := nearestLarger(a, 0)
	if !ok {
		fmt.Println("null")
	} else {
		fmt.Println(idx) // expected 3
	}
}
