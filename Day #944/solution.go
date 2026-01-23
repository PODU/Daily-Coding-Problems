// Day 944: Next greater permutation of an integer's digits (in-place on digit array).
// Find pivot, swap with next larger from the right, reverse suffix. Time O(d), Space O(d).
package main

import (
	"fmt"
	"strconv"
)

// Returns -1 if no greater permutation exists.
func nextPermutation(num int64) int64 {
	d := []byte(strconv.FormatInt(num, 10))
	n := len(d)
	i := n - 2
	for i >= 0 && d[i] >= d[i+1] {
		i--
	}
	if i < 0 {
		return -1
	}
	j := n - 1
	for d[j] <= d[i] {
		j--
	}
	d[i], d[j] = d[j], d[i]
	for l, r := i+1, n-1; l < r; l, r = l+1, r-1 {
		d[l], d[r] = d[r], d[l]
	}
	res, _ := strconv.ParseInt(string(d), 10, 64)
	return res
}

func main() {
	fmt.Println(nextPermutation(48975)) // 49578
}
