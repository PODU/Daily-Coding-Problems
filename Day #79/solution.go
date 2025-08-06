// Day 79: Can array become non-decreasing with at most one modification?
// Greedy: on each violation, prefer lowering nums[i]; allow only one fix. Time O(n), Space O(1).
package main

import "fmt"

func checkPossibility(input []int) bool {
	nums := append([]int(nil), input...)
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
	fmt.Println(checkPossibility([]int{10, 5, 7})) // true
	fmt.Println(checkPossibility([]int{10, 5, 1})) // false
}
