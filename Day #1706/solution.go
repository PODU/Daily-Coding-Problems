// Longest consecutive sequence: hash set, count runs from each sequence start. O(n) time, O(n) space.
package main

import "fmt"

func longestConsecutive(nums []int) int {
	s := make(map[int]bool)
	for _, n := range nums {
		s[n] = true
	}
	best := 0
	for n := range s {
		if !s[n-1] {
			cur, length := n, 1
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
