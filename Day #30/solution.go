// Trapping rain water with two pointers.
// Time: O(n), Space: O(1).
package main

import "fmt"

func trap(h []int) int {
	l, r := 0, len(h)-1
	leftMax, rightMax, water := 0, 0, 0
	for l < r {
		if h[l] < h[r] {
			if h[l] > leftMax {
				leftMax = h[l]
			}
			water += leftMax - h[l]
			l++
		} else {
			if h[r] > rightMax {
				rightMax = h[r]
			}
			water += rightMax - h[r]
			r--
		}
	}
	return water
}

func main() {
	heights := []int{3, 0, 1, 3, 0, 5}
	fmt.Println(trap(heights))
}
