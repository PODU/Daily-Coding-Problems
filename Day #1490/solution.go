// Find the duplicate in array of length n+1 with values in {1..n}.
// Floyd's tortoise-and-hare cycle detection. Time O(n), Space O(1).
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
	nums := []int{1, 2, 3, 4, 2} // n = 4
	fmt.Println(findDuplicate(nums)) // expected: 2
}
