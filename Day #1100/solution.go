// Day 1100: Search sorted array in O(log N) using only addition/comparison
// (no *, /, or bit-shift). Binary lifting with powers of two built by doubling.
// Time: O(log N). Space: O(log N).
package main

import "fmt"

func contains(a []int, x int) bool {
	n := len(a)
	if n == 0 {
		return false
	}
	var pows []int
	for p := 1; p <= n; p += p {
		pows = append(pows, p)
	}
	pos := -1
	for i := len(pows) - 1; i >= 0; i-- {
		p := pows[i]
		if pos+p < n && a[pos+p] <= x {
			pos += p
		}
	}
	return pos >= 0 && a[pos] == x
}

func main() {
	a := []int{1, 3, 5, 7, 9, 11}
	fmt.Println(contains(a, 7)) // true
	fmt.Println(contains(a, 8)) // false
}
