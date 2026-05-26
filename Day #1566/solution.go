// Longest palindromic substring via Manacher's algorithm. Time O(n), space O(n).
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
	center, right := 0, 0
	for i := 1; i < n-1; i++ {
		mirror := 2*center - i
		if i < right {
			if right-i < p[mirror] {
				p[i] = right - i
			} else {
				p[i] = p[mirror]
			}
		}
		for t[i+p[i]+1] == t[i-p[i]-1] {
			p[i]++
		}
		if i+p[i] > right {
			center, right = i, i+p[i]
		}
	}
	maxLen, centerIndex := 0, 0
	for i := 1; i < n-1; i++ {
		if p[i] > maxLen {
			maxLen, centerIndex = p[i], i
		}
	}
	start := (centerIndex - maxLen) / 2
	return s[start : start+maxLen]
}

func main() {
	fmt.Println(longestPalindrome("aabcdcb"))
}
