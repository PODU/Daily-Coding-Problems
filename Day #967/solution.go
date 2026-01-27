// Day 967: Find the duplicate in array of n+1 ints from {1..n}.
// Approach: Floyd's tortoise & hare on value->index cycle. Time O(n), Space O(1).
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
	fmt.Println(findDuplicate([]int{1, 3, 4, 2, 2})) // 2
}
