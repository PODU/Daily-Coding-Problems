// Product of array except self via prefix and suffix passes (no division).
// Time: O(n), Space: O(1) extra (excluding output).
package main

import "fmt"

func productExceptSelf(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	pre := 1
	for i := 0; i < n; i++ {
		res[i] = pre
		pre *= nums[i]
	}
	suf := 1
	for i := n - 1; i >= 0; i-- {
		res[i] *= suf
		suf *= nums[i]
	}
	return res
}

func main() {
	fmt.Println(productExceptSelf([]int{1, 2, 3, 4, 5}))
}
