// Day 181: Minimum palindrome partitioning.
// DP: palindrome table + min-cut DP with reconstruction. Time O(n^2), Space O(n^2).
package main

import (
	"fmt"
	"strings"
)

func minPalindromePartition(s string) []string {
	n := len(s)
	if n == 0 {
		return []string{}
	}
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
		pal[i][i] = true
	}
	for L := 2; L <= n; L++ {
		for i := 0; i+L-1 < n; i++ {
			j := i + L - 1
			if s[i] == s[j] && (L == 2 || pal[i+1][j-1]) {
				pal[i][j] = true
			}
		}
	}
	const INF = 1 << 30
	cut := make([]int, n+1)
	prev := make([]int, n+1)
	for i := range cut {
		cut[i] = INF
		prev[i] = -1
	}
	cut[0] = 0
	for i := 1; i <= n; i++ {
		for j := 0; j < i; j++ {
			if pal[j][i-1] && cut[j]+1 < cut[i] {
				cut[i] = cut[j] + 1
				prev[i] = j
			}
		}
	}
	var res []string
	for i := n; i > 0; i = prev[i] {
		res = append([]string{s[prev[i]:i]}, res...)
	}
	return res
}

func fmtRes(v []string) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = "\"" + x + "\""
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(fmtRes(minPalindromePartition("racecarannakayak")))
	fmt.Println(fmtRes(minPalindromePartition("abc")))
}
