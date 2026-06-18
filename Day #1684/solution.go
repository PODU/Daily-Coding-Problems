// Day 1684: Levenshtein edit distance via 1D rolling DP.
// Time O(n*m), Space O(min(n,m)).
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
	n, m := len(a), len(b)
	dp := make([]int, m+1)
	for j := 0; j <= m; j++ {
		dp[j] = j
	}
	for i := 1; i <= n; i++ {
		prev := dp[0]
		dp[0] = i
		for j := 1; j <= m; j++ {
			tmp := dp[j]
			if a[i-1] == b[j-1] {
				dp[j] = prev
			} else {
				dp[j] = 1 + min3(prev, dp[j], dp[j-1])
			}
			prev = tmp
		}
	}
	return dp[m]
}

func main() {
	fmt.Println(editDistance("kitten", "sitting")) // 3
}
