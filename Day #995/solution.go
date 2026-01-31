// Day 995: Product of array except self, without division.
// Left pass stores prefix products; right pass multiplies by suffix products.
// O(n) time, O(1) extra space (besides output).
package main

import "fmt"

func products(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	left := 1
	for i := 0; i < n; i++ {
		res[i] = left
		left *= nums[i]
	}
	right := 1
	for i := n - 1; i >= 0; i-- {
		res[i] *= right
		right *= nums[i]
	}
	return res
}

func main() {
	fmt.Println(products([]int{1, 2, 3, 4, 5})) // [120 60 40 30 24]
	fmt.Println(products([]int{3, 2, 1}))       // [2 3 6]
}
