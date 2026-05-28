// Day 1578: Find min and max using ~3N/2 comparisons (pairwise method).
// Compare elements in pairs, then each pair contributes one compare to min and one to max.
// Time: O(N) with ceil(3N/2)-2 comparisons; Space: O(1).
package main

import "fmt"

func minMax(a []int) (int, int) {
	n := len(a)
	var mn, mx, i int
	if n&1 == 1 {
		mn, mx, i = a[0], a[0], 1
	} else {
		if a[0] < a[1] {
			mn, mx = a[0], a[1]
		} else {
			mn, mx = a[1], a[0]
		}
		i = 2
	}
	for ; i+1 < n; i += 2 {
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
	mn, mx := minMax([]int{3, 5, 1, 2, 8, 7, 4})
	fmt.Printf("min=%d max=%d\n", mn, mx) // min=1 max=8
}
