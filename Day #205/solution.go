// Day 205: Next permutation of an integer's digits.
// Standard next-permutation: find rightmost ascent, swap with next-larger suffix digit, reverse suffix.
// Time: O(d), Space: O(d) for digits.
package main

import (
	"fmt"
	"strconv"
)

func nextPermutation(n int64) int64 {
	s := []byte(strconv.FormatInt(n, 10))
	i := len(s) - 2
	for i >= 0 && s[i] >= s[i+1] {
		i--
	}
	if i < 0 {
		return n // already the largest permutation
	}
	j := len(s) - 1
	for s[j] <= s[i] {
		j--
	}
	s[i], s[j] = s[j], s[i]
	for l, r := i+1, len(s)-1; l < r; l, r = l+1, r-1 {
		s[l], s[r] = s[r], s[l]
	}
	v, _ := strconv.ParseInt(string(s), 10, 64)
	return v
}

func main() {
	fmt.Println(nextPermutation(48975)) // 49578
}
