// Day 1804: Find a duplicate in array of n+1 elements from {1..n} using a seen map.
// O(n) time, O(n) space.
package main

import "fmt"

func findDuplicate(nums []int) int {
	seen := make(map[int]bool, len(nums))
	for _, x := range nums {
		if seen[x] {
			return x
		}
		seen[x] = true
	}
	return -1 // no duplicate (won't happen per problem constraints)
}

func main() {
	nums := []int{1, 3, 4, 2, 2}
	fmt.Println(findDuplicate(nums)) // expected 2
}
