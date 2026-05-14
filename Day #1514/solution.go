// Sliding window with last-seen index map; advance left past duplicates, track max window length.
// Time: O(n), Space: O(n).
package main

import "fmt"

func longestDistinct(a []int) int {
	last := make(map[int]int)
	best, left := 0, 0
	for r, v := range a {
		if prev, ok := last[v]; ok && prev >= left {
			left = prev + 1
		}
		last[v] = r
		if r-left+1 > best {
			best = r - left + 1
		}
	}
	return best
}

func main() {
	a := []int{5, 1, 3, 5, 2, 3, 4, 1}
	fmt.Println(longestDistinct(a))
}
