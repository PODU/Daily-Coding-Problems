// Day 768: Find min and max together using ~3N/2 comparisons (< 2*(N-2)).
// Process elements in pairs: compare the pair, then smaller vs min, larger vs max.
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
	for ; i < n; i += 2 {
		lo, hi := a[i], a[i+1]
		if lo > hi {
			lo, hi = hi, lo
		}
		if lo < mn {
			mn = lo
		}
		if hi > mx {
			mx = hi
		}
	}
	return mn, mx
}

func main() {
	mn, mx := minMax([]int{3, 5, 1, 2, 4, 8, 7})
	fmt.Printf("min=%d max=%d\n", mn, mx) // min=1 max=8
}
