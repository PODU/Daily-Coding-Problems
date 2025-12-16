// Longest palindromic substring via Manacher's algorithm.
// Transform with '#' separators, expand radii using mirror symmetry.
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
	t := "#" + strings.Join(strings.Split(s, ""), "#") + "#"
	n := len(t)
	p := make([]int, n)
	c, r := 0, 0
	for i := 0; i < n; i++ {
		if i < r {
			if r-i < p[2*c-i] {
				p[i] = r - i
			} else {
				p[i] = p[2*c-i]
			}
		}
		for i-p[i]-1 >= 0 && i+p[i]+1 < n && t[i-p[i]-1] == t[i+p[i]+1] {
			p[i]++
		}
		if i+p[i] > r {
			c, r = i, i+p[i]
		}
	}
	maxLen, center := 0, 0
	for i := 0; i < n; i++ {
		if p[i] > maxLen {
			maxLen, center = p[i], i
		}
	}
	start := (center - maxLen) / 2
	return s[start : start+maxLen]
}

func main() {
	fmt.Println(longestPalindrome("aabcdcb")) // bcdcb
	fmt.Println(longestPalindrome("bananas")) // anana
}
