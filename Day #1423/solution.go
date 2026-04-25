// Day 1423: can you reach the end of the array (each value = max steps forward)?
// Approach: greedy, track farthest reachable index. O(n) time, O(1) space.
package main

import "fmt"

func canReachEnd(nums []int) bool {
	farthest := 0
	for i, step := range nums {
		if i > farthest {
			return false
		}
		if i+step > farthest {
			farthest = i + step
		}
	}
	return true
}

func main() {
	fmt.Println(canReachEnd([]int{1, 3, 1, 2, 0, 1})) // true
	fmt.Println(canReachEnd([]int{1, 2, 1, 0, 0}))    // false
}
