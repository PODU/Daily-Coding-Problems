// Largest rectangle in histogram via monotonic increasing stack. Time O(N), Space O(N).
package main

import "fmt"

func largestRectangle(heights []int) int {
	stack := []int{} // indices of increasing bars
	best := 0
	n := len(heights)
	for i := 0; i <= n; i++ {
		h := 0
		if i < n {
			h = heights[i]
		}
		for len(stack) > 0 && heights[stack[len(stack)-1]] >= h {
			height := heights[stack[len(stack)-1]]
			stack = stack[:len(stack)-1]
			width := i
			if len(stack) > 0 {
				width = i - stack[len(stack)-1] - 1
			}
			if area := height * width; area > best {
				best = area
			}
		}
		stack = append(stack, i)
	}
	return best
}

func main() {
	fmt.Println(largestRectangle([]int{1, 3, 2, 5}))
}
