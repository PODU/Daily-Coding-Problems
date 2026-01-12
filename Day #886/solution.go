// Edit distance (Levenshtein) via DP. Time O(n*m), Space O(min(n,m)).
package main

import "fmt"

func min3(a, b, c int) int {
	if b < a {
		a = b
	}
	if c < a {
		a = c
	}
	return a
}

func editDistance(a, b string) int {
	if len(a) > len(b) {
		a, b = b, a
	}
	n, m := len(a), len(b)
	prev := make([]int, n+1)
	cur := make([]int, n+1)
	for i := 0; i <= n; i++ {
		prev[i] = i
	}
	for j := 1; j <= m; j++ {
		cur[0] = j
		for i := 1; i <= n; i++ {
			cost := 1
			if a[i-1] == b[j-1] {
				cost = 0
			}
			cur[i] = min3(prev[i]+1, cur[i-1]+1, prev[i-1]+cost)
		}
		prev, cur = cur, prev
	}
	return prev[n]
}

func main() {
	fmt.Println(editDistance("kitten", "sitting"))
}
