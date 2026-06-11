// Next permutation of the digit array: find rightmost ascending pair, swap with
// next-greater suffix digit, reverse suffix. Time O(d), Space O(d).
package main

import (
	"fmt"
	"strconv"
)

func nextPermutation(num int) int {
	s := []byte(strconv.Itoa(num))
	n := len(s)
	i := n - 2
	for i >= 0 && s[i] >= s[i+1] {
		i--
	}
	if i < 0 {
		return -1 // no next permutation
	}
	j := n - 1
	for s[j] <= s[i] {
		j--
	}
	s[i], s[j] = s[j], s[i]
	for l, r := i+1, n-1; l < r; l, r = l+1, r-1 {
		s[l], s[r] = s[r], s[l]
	}
	v, _ := strconv.Atoi(string(s))
	return v
}

func main() {
	fmt.Println(nextPermutation(48975))
}
