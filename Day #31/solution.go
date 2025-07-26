// Edit Distance via DP. Time O(m*n), Space O(min(m,n)) using two rolling rows.
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
	if len(a) < len(b) {
		a, b = b, a
	}
	m, n := len(a), len(b)
	prev := make([]int, n+1)
	cur := make([]int, n+1)
	for j := 0; j <= n; j++ {
		prev[j] = j
	}
	for i := 1; i <= m; i++ {
		cur[0] = i
		for j := 1; j <= n; j++ {
			if a[i-1] == b[j-1] {
				cur[j] = prev[j-1]
			} else {
				cur[j] = 1 + min3(prev[j-1], prev[j], cur[j-1])
			}
		}
		prev, cur = cur, prev
	}
	return prev[n]
}

func main() {
	fmt.Println(editDistance("kitten", "sitting"))
}
