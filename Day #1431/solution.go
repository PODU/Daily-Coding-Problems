// Day 1431: Majority element (appears > floor(n/2)).
// Approach: Boyer-Moore voting. Time: O(n), Space: O(1).
package main

import "fmt"

func majorityElement(nums []int) int {
	count, candidate := 0, 0
	for _, x := range nums {
		if count == 0 {
			candidate = x
		}
		if x == candidate {
			count++
		} else {
			count--
		}
	}
	return candidate
}

func main() {
	fmt.Println(majorityElement([]int{1, 2, 1, 1, 3, 4, 0})) // 1
}
