// Levenshtein edit distance via DP with rolling array.
// Time O(m*n), Space O(min(m,n)).
package main

import "fmt"

func editDistance(a, b string) int {
	if len(a) < len(b) {
		a, b = b, a
	}
	n := len(b)
	prev := make([]int, n+1)
	cur := make([]int, n+1)
	for j := 0; j <= n; j++ {
		prev[j] = j
	}
	for i := 1; i <= len(a); i++ {
		cur[0] = i
		for j := 1; j <= n; j++ {
			cost := 1
			if a[i-1] == b[j-1] {
				cost = 0
			}
			cur[j] = min3(prev[j]+1, cur[j-1]+1, prev[j-1]+cost)
		}
		prev, cur = cur, prev
	}
	return prev[n]
}

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

func main() {
	fmt.Println(editDistance("kitten", "sitting"))
}
