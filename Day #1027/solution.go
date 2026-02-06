// Day 1027: Longest consecutive elements sequence.
// Hash set; only count runs starting at numbers with no predecessor. Time O(n), Space O(n).
package main

import "fmt"

func longestConsecutive(nums []int) int {
	s := make(map[int]bool)
	for _, x := range nums {
		s[x] = true
	}
	best := 0
	for x := range s {
		if !s[x-1] {
			length, cur := 1, x
			for s[cur+1] {
				cur++
				length++
			}
			if length > best {
				best = length
			}
		}
	}
	return best
}

func main() {
	fmt.Println(longestConsecutive([]int{100, 4, 200, 1, 3, 2}))
}
