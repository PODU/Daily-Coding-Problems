// Day 1844: Largest rectangle in a histogram via a monotonic increasing stack.
// Time O(N), Space O(N).
package main

import "fmt"

func largestRectangle(heights []int) int {
	h := append(append([]int{}, heights...), 0) // sentinel
	stack := []int{}
	best := 0
	for i := 0; i < len(h); i++ {
		for len(stack) > 0 && h[stack[len(stack)-1]] >= h[i] {
			height := h[stack[len(stack)-1]]
			stack = stack[:len(stack)-1]
			left := -1
			if len(stack) > 0 {
				left = stack[len(stack)-1]
			}
			width := i - left - 1
			if area := height * width; area > best {
				best = area
			}
		}
		stack = append(stack, i)
	}
	return best
}

func main() {
	fmt.Println(largestRectangle([]int{1, 3, 2, 5})) // 6
}
