// Minimum jumps to reach end: greedy BFS-by-levels tracking current reach and farthest reach.
// Bump jumps when current window ends. Time O(n), Space O(1).
package main

import "fmt"

func minJumps(nums []int) int {
	n := len(nums)
	if n <= 1 {
		return 0
	}
	jumps, curEnd, farthest := 0, 0, 0
	for i := 0; i < n-1; i++ {
		if i+nums[i] > farthest {
			farthest = i + nums[i]
		}
		if i == curEnd {
			jumps++
			curEnd = farthest
		}
	}
	return jumps
}

func main() {
	nums := []int{6, 2, 4, 0, 5, 1, 1, 4, 2, 9}
	fmt.Println(minJumps(nums))
}
