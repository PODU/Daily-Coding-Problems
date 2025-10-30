// Longest consecutive sequence: hash set, extend only from sequence starts. O(n) time/space.
package main

import "fmt"

func longestConsecutive(a []int) int {
	s := make(map[int]bool)
	for _, x := range a {
		s[x] = true
	}
	best := 0
	for x := range s {
		if s[x-1] {
			continue
		}
		length, cur := 1, x
		for s[cur+1] {
			cur++
			length++
		}
		if length > best {
			best = length
		}
	}
	return best
}

func main() {
	fmt.Println(longestConsecutive([]int{100, 4, 200, 1, 3, 2}))
}
