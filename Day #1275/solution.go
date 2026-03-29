// Day 1275: Longest palindromic substring via Manacher's algorithm. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strings"
)

func longestPalindrome(s string) string {
	if len(s) == 0 {
		return ""
	}
	var sb strings.Builder
	sb.WriteString("^")
	for _, c := range s {
		sb.WriteString("#")
		sb.WriteRune(c)
	}
	sb.WriteString("#$")
	t := []rune(sb.String())
	n := len(t)
	p := make([]int, n)
	center, right := 0, 0
	for i := 1; i < n-1; i++ {
		if i < right {
			if m := right - i; p[2*center-i] < m {
				p[i] = p[2*center-i]
			} else {
				p[i] = m
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
	return string([]rune(s)[start : start+maxLen])
}

func main() {
	fmt.Printf("%q\n", longestPalindrome("aabcdcb")) // "bcdcb"
	fmt.Printf("%q\n", longestPalindrome("bananas")) // "anana"
}
