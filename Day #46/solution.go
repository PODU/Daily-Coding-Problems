// Day 46: Longest palindromic substring via Manacher's algorithm.
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strings"
)

func longestPalindrome(s string) string {
	if len(s) == 0 {
		return ""
	}
	t := "^#" + strings.Join(strings.Split(s, ""), "#") + "#$"
	n := len(t)
	p := make([]int, n)
	c, r := 0, 0
	for i := 1; i < n-1; i++ {
		if i < r {
			if r-i < p[2*c-i] {
				p[i] = r - i
			} else {
				p[i] = p[2*c-i]
			}
		}
		for t[i+1+p[i]] == t[i-1-p[i]] {
			p[i]++
		}
		if i+p[i] > r {
			c, r = i, i+p[i]
		}
	}
	maxLen, center := 0, 0
	for i := 1; i < n-1; i++ {
		if p[i] > maxLen {
			maxLen, center = p[i], i
		}
	}
	start := (center - maxLen) / 2
	return s[start : start+maxLen]
}

func main() {
	fmt.Printf("\"%s\"\n", longestPalindrome("aabcdcb"))
	fmt.Printf("\"%s\"\n", longestPalindrome("bananas"))
}
