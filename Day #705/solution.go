// Day 705: Trapping rain water.
// Approach: two pointers tracking running left/right maxima; the smaller side is
// bounded so we can resolve it. Time O(N), Space O(1).
package main

import "fmt"

func trap(h []int) int {
	l, r := 0, len(h)-1
	lmax, rmax, water := 0, 0, 0
	for l < r {
		if h[l] < h[r] {
			if h[l] > lmax {
				lmax = h[l]
			}
			water += lmax - h[l]
			l++
		} else {
			if h[r] > rmax {
				rmax = h[r]
			}
			water += rmax - h[r]
			r--
		}
	}
	return water
}

func main() {
	fmt.Println(trap([]int{2, 1, 2}))          // 1
	fmt.Println(trap([]int{3, 0, 1, 3, 0, 5})) // 8
}
