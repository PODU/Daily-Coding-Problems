// Day 164: Find duplicate via Floyd's cycle detection (values as next-pointers).
// O(n) time, O(1) extra space (beats the O(n)-space requirement).
package main

import "fmt"

func findDuplicate(nums []int) int {
	slow, fast := nums[0], nums[0]
	for {
		slow = nums[slow]
		fast = nums[nums[fast]]
		if slow == fast {
			break
		}
	}
	slow = nums[0]
	for slow != fast {
		slow = nums[slow]
		fast = nums[fast]
	}
	return slow
}

func main() {
	nums := []int{1, 2, 3, 4, 2} // n = 4, length n+1
	fmt.Println(findDuplicate(nums)) // 2
}
