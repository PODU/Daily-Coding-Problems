// Two numbers summing to k via one-pass hash set. Time O(n), Space O(n).
package main

import "fmt"

func twoSum(nums []int, k int) bool {
	seen := make(map[int]bool)
	for _, x := range nums {
		if seen[k-x] {
			return true
		}
		seen[x] = true
	}
	return false
}

func main() {
	nums := []int{10, 15, 3, 7}
	fmt.Println(twoSum(nums, 17))
}
