// Find min and max using ~3*ceil(N/2) comparisons (pairwise method).
// Time: O(N) with <2N comparisons. Space: O(1).
package main

import "fmt"

func minMax(a []int) (int, int, int) {
	cmps := 0
	n := len(a)
	var mn, mx, i int
	if n%2 == 1 {
		mn, mx, i = a[0], a[0], 1 // odd: seed with first
	} else { // even: seed with first pair
		cmps++
		if a[0] < a[1] {
			mn, mx = a[0], a[1]
		} else {
			mn, mx = a[1], a[0]
		}
		i = 2
	}
	for ; i+1 < n; i += 2 {
		var lo, hi int
		cmps++
		if a[i] < a[i+1] {
			lo, hi = a[i], a[i+1]
		} else {
			lo, hi = a[i+1], a[i]
		}
		cmps++
		if lo < mn {
			mn = lo
		}
		cmps++
		if hi > mx {
			mx = hi
		}
	}
	return mn, mx, cmps
}

func main() {
	a := []int{7, 2, 9, 4, 1, 8, 3}
	mn, mx, _ := minMax(a)
	fmt.Printf("min=%d max=%d\n", mn, mx)
}
