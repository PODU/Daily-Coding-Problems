// Trapping rain water via two pointers tracking leftMax/rightMax. Time O(N), Space O(1).
package main

import "fmt"

func trap(h []int) int {
	l, r := 0, len(h)-1
	lm, rm, water := 0, 0, 0
	for l < r {
		if h[l] < h[r] {
			if h[l] > lm {
				lm = h[l]
			}
			water += lm - h[l]
			l++
		} else {
			if h[r] > rm {
				rm = h[r]
			}
			water += rm - h[r]
			r--
		}
	}
	return water
}

func main() {
	fmt.Printf("[2, 1, 2] -> %d\n", trap([]int{2, 1, 2}))
	fmt.Printf("[3, 0, 1, 3, 0, 5] -> %d\n", trap([]int{3, 0, 1, 3, 0, 5}))
}
