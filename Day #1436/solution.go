// Day 1436: Length of longest subarray with all distinct elements.
// Approach: Sliding window with last-seen index map; shrink left on repeat.
// Time: O(n), Space: O(n).
package main

import "fmt"

func longestDistinct(arr []int) int {
	last := map[int]int{}
	best, left := 0, 0
	for right, v := range arr {
		if prev, ok := last[v]; ok && prev >= left {
			left = prev + 1
		}
		last[v] = right
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}

func main() {
	fmt.Println(longestDistinct([]int{5, 1, 3, 5, 2, 3, 4, 1})) // 5
}
