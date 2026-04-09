// Day 1328: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i]=min pieces for prefix i, with parent pointers to rebuild. O(n^2) time/space.
package main

import "fmt"

func minPalindromePartition(s string) []string {
	n := len(s)
	if n == 0 {
		return []string{}
	}
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
	}
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			pal[i][j] = s[i] == s[j] && (j-i < 2 || pal[i+1][j-1])
		}
	}

	const inf = 1 << 30
	dp := make([]int, n+1)
	prev := make([]int, n+1)
	for i := range dp {
		dp[i] = inf
	}
	dp[0] = 0
	for i := 1; i <= n; i++ {
		for j := 0; j < i; j++ {
			if pal[j][i-1] && dp[j]+1 < dp[i] {
				dp[i] = dp[j] + 1
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

func main() {
	fmt.Println(minPalindromePartition("racecarannakayak")) // [racecar anna kayak]
	fmt.Println(minPalindromePartition("abc"))               // [a b c]
}
