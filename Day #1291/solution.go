// Day 1291: Next permutation of an integer's digits.
// Standard next-permutation: scan from right, swap, reverse suffix. O(D) time, O(D) space.
package main

import "fmt"

func nextPermutation(num string) string {
	s := []byte(num)
	n := len(s)
	i := n - 2
	for i >= 0 && s[i] >= s[i+1] {
		i--
	}
	if i < 0 {
		return num // already largest
	}
	j := n - 1
	for s[j] <= s[i] {
		j--
	}
	s[i], s[j] = s[j], s[i]
	for l, r := i+1, n-1; l < r; l, r = l+1, r-1 {
		s[l], s[r] = s[r], s[l]
	}
	return string(s)
}

func main() {
	fmt.Println(nextPermutation("48975")) // 49578
}
