// Day 1421: product of all elements except self, without division.
// Approach: prefix products pass then suffix products pass. O(n) time, O(1) extra (besides output).
package main

import "fmt"

func productExceptSelf(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	prefix := 1
	for i := 0; i < n; i++ {
		res[i] = prefix
		prefix *= nums[i]
	}
	suffix := 1
	for i := n - 1; i >= 0; i-- {
		res[i] *= suffix
		suffix *= nums[i]
	}
	return res
}

func main() {
	fmt.Println(productExceptSelf([]int{1, 2, 3, 4, 5})) // [120 60 40 30 24]
	fmt.Println(productExceptSelf([]int{3, 2, 1}))       // [2 3 6]
}
