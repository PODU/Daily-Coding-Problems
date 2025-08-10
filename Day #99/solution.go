// Day 99: Longest consecutive sequence. Hash all values; begin counting only at
// sequence starts (n-1 absent) and walk up. O(n) time, O(n) space.
package main

import "fmt"

func longestConsecutive(nums []int) int {
	s := make(map[int]bool, len(nums))
	for _, n := range nums {
		s[n] = true
	}
	best := 0
	for n := range s {
		if !s[n-1] {
			length := 1
			for s[n+length] {
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
	fmt.Println(longestConsecutive([]int{100, 4, 200, 1, 3, 2})) // 4
}
