// Min & Max in ~3N/2 comparisons: process elements in pairs, compare the pair first,
// then smaller vs min and larger vs max. Time: O(N), Space: O(1). Comparisons ~ 3*ceil(N/2)-2.
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
	for i < n {
		x, y := a[i], a[i+1]
		if x < y {
			if x < mn {
				mn = x
			}
			if y > mx {
				mx = y
			}
		} else {
			if y < mn {
				mn = y
			}
			if x > mx {
				mx = x
			}
		}
		i += 2
	}
	return mn, mx
}

func main() {
	a := []int{3, 5, 1, 2, 4, 8, 7}
	mn, mx := minMax(a)
	fmt.Printf("min=%d max=%d\n", mn, mx) // min=1 max=8
}
