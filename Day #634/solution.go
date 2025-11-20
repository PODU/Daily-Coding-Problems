// Day 634: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to settle areas.
// Time: O(N), Space: O(N).
package main

import "fmt"

func largestRectangle(h []int) int {
	st := []int{}
	best := 0
	n := len(h)
	for i := 0; i <= n; i++ {
		cur := 0
		if i < n {
			cur = h[i]
		}
		for len(st) > 0 && h[st[len(st)-1]] >= cur {
			height := h[st[len(st)-1]]
			st = st[:len(st)-1]
			left := -1
			if len(st) > 0 {
				left = st[len(st)-1]
			}
			width := i - left - 1
			if height*width > best {
				best = height * width
			}
		}
		st = append(st, i)
	}
	return best
}

func main() {
	fmt.Println(largestRectangle([]int{1, 3, 2, 5})) // 6
}
