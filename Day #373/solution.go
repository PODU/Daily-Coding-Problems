// Day 373: Longest run of consecutive integers formable from the list.
// Hash set; start counting only at run-starts (x with no x-1 present). O(n) time.
package main

import "fmt"

func longestConsecutive(nums []int) int {
	s := make(map[int]bool, len(nums))
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
	fmt.Println(longestConsecutive([]int{5, 2, 99, 3, 4, 1, 100})) // 5
}
