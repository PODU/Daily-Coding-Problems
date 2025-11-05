// Non-decreasing with at most one modification: single pass, greedy fix. O(n) time, O(1) space.
package main

import "fmt"

func checkPossibility(in []int) bool {
	nums := append([]int(nil), in...)
	cnt := 0
	for i := 1; i < len(nums); i++ {
		if nums[i] < nums[i-1] {
			cnt++
			if cnt > 1 {
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
	fmt.Println(checkPossibility([]int{10, 5, 7}))
	fmt.Println(checkPossibility([]int{10, 5, 1}))
}
