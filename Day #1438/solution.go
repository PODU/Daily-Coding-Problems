// Day 1438: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to compute areas.
// Time: O(n), Space: O(n).
package main

import "fmt"

func largestRectangle(heights []int) int {
	st := []int{} // indices with increasing heights
	best := 0
	n := len(heights)
	for i := 0; i <= n; i++ {
		h := 0
		if i < n {
			h = heights[i]
		}
		for len(st) > 0 && heights[st[len(st)-1]] >= h {
			top := st[len(st)-1]
			st = st[:len(st)-1]
			width := i
			if len(st) > 0 {
				width = i - st[len(st)-1] - 1
			}
			if area := heights[top] * width; area > best {
				best = area
			}
		}
		st = append(st, i)
	}
	return best
}

func main() {
	fmt.Println(largestRectangle([]int{1, 3, 2, 5})) // 6
}
