// Day 489: Longest subarray with all distinct elements.
// Sliding window + last-seen map; move left past previous occurrence. Time O(n), Space O(n).
package main

import "fmt"

func longestDistinctSubarray(a []int) int {
	last := make(map[int]int)
	left, best := 0, 0
	for right, val := range a {
		if prev, ok := last[val]; ok && prev >= left {
			left = prev + 1
		}
		last[val] = right
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}

func main() {
	fmt.Println(longestDistinctSubarray([]int{5, 1, 3, 5, 2, 3, 4, 1})) // 5
}
