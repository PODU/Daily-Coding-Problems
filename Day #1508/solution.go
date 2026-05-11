// Product of array except self without division: prefix-product then suffix-product.
// Two passes, output array only. Time O(n), Space O(1) extra (besides output).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func productExceptSelf(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	res[0] = 1
	for i := 1; i < n; i++ {
		res[i] = res[i-1] * nums[i-1]
	}
	suffix := 1
	for i := n - 1; i >= 0; i-- {
		res[i] *= suffix
		suffix *= nums[i]
	}
	return res
}

func main() {
	nums := []int{1, 2, 3, 4, 5}
	res := productExceptSelf(nums)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
