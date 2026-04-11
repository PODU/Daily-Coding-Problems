// Longest consecutive run via hash set: start only at run heads (x-1 absent), walk up. O(n) time, O(n) space.
package main

import "fmt"

func longestConsecutive(nums []int) int {
	set := make(map[int]bool, len(nums))
	for _, x := range nums {
		set[x] = true
	}
	best := 0
	for x := range set {
		if set[x-1] {
			continue // not a run head
		}
		cur, length := x, 1
		for set[cur+1] {
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
	nums := []int{100, 4, 200, 1, 3, 2}
	fmt.Println(longestConsecutive(nums))
}
