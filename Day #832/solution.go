// Longest subarray with all distinct elements.
// Sliding window with last-seen index map. Time: O(N), Space: O(N).
package main

import "fmt"

func longestDistinct(a []int) int {
	last := make(map[int]int)
	best, start := 0, 0
	for i, x := range a {
		if p, ok := last[x]; ok && p >= start {
			start = p + 1
		}
		last[x] = i
		if i-start+1 > best {
			best = i - start + 1
		}
	}
	return best
}

func main() {
	a := []int{5, 1, 3, 5, 2, 3, 4, 1}
	fmt.Println(longestDistinct(a))
}
