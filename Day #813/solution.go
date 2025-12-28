// Non-decreasing with at most 1 modification: single pass counting violations,
// greedily lower nums[i-1] or raise nums[i]. Time O(n), Space O(1).
package main

import "fmt"

func canBeNonDecreasing(input []int) bool {
	nums := make([]int, len(input))
	copy(nums, input)
	count := 0
	for i := 1; i < len(nums); i++ {
		if nums[i-1] > nums[i] {
			count++
			if count > 1 {
				return false
			}
			if i < 2 || nums[i-2] <= nums[i] {
				nums[i-1] = nums[i]
			} else {
				nums[i] = nums[i-1]
			}
		}
	}
	return true
}

func main() {
	fmt.Println(canBeNonDecreasing([]int{10, 5, 7}))
	fmt.Println(canBeNonDecreasing([]int{10, 5, 1}))
}
