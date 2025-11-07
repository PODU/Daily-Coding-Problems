// Product of array except self without division.
// Prefix * suffix products in two passes. Time O(N), Space O(1) extra (besides output).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func productExceptSelf(a []int64) []int64 {
	n := len(a)
	res := make([]int64, n)
	var prefix int64 = 1
	for i := 0; i < n; i++ {
		res[i] = prefix
		prefix *= a[i]
	}
	var suffix int64 = 1
	for i := n - 1; i >= 0; i-- {
		res[i] *= suffix
		suffix *= a[i]
	}
	return res
}

func format(v []int64) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = strconv.FormatInt(x, 10)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(format(productExceptSelf([]int64{1, 2, 3, 4, 5})))
	fmt.Println(format(productExceptSelf([]int64{3, 2, 1})))
}
