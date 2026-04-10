// Day 1334: Levenshtein edit distance between two strings.
// Classic DP with rolling row. O(n*m) time, O(min(n,m)) space.
package main

import "fmt"

func min3(a, b, c int) int {
	m := a
	if b < m {
		m = b
	}
	if c < m {
		m = c
	}
	return m
}

func editDistance(a, b string) int {
	n, m := len(a), len(b)
	prev := make([]int, m+1)
	cur := make([]int, m+1)
	for j := 0; j <= m; j++ {
		prev[j] = j
	}
	for i := 1; i <= n; i++ {
		cur[0] = i
		for j := 1; j <= m; j++ {
			if a[i-1] == b[j-1] {
				cur[j] = prev[j-1]
			} else {
				cur[j] = 1 + min3(prev[j-1], prev[j], cur[j-1])
			}
		}
		prev, cur = cur, prev
	}
	return prev[m]
}

func main() {
	fmt.Println(editDistance("kitten", "sitting")) // 3
}
