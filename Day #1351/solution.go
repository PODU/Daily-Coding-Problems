// Pairwise min/max: process pairs, compare smaller->min, larger->max -> ~3N/2 comparisons.
// Time: O(N) (~3N/2 comparisons), Space: O(1).
package main

import "fmt"

func minMax(a []int) (int, int) {
	n := len(a)
	var mn, mx, i int
	if n%2 == 0 {
		if a[0] < a[1] {
			mn, mx = a[0], a[1]
		} else {
			mn, mx = a[1], a[0]
		}
		i = 2
	} else {
		mn, mx = a[0], a[0]
		i = 1
	}
	for ; i+1 <= n; i += 2 {
		var small, large int
		if a[i] < a[i+1] {
			small, large = a[i], a[i+1]
		} else {
			small, large = a[i+1], a[i]
		}
		if small < mn {
			mn = small
		}
		if large > mx {
			mx = large
		}
	}
	return mn, mx
}

func main() {
	a := []int{3, 5, 1, 2, 4, 8, 7}
	mn, mx := minMax(a)
	fmt.Printf("Min: %d, Max: %d\n", mn, mx)
}
