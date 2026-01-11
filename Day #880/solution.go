// Min palindrome partition via DP with palindrome table + reconstruction.
// Time O(n^2), Space O(n^2).
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
			if s[i] == s[j] && (j-i < 2 || pal[i+1][j-1]) {
				pal[i][j] = true
			}
		}
	}

	const INF = 1 << 30
	dp := make([]int, n+1)
	cut := make([]int, n+1)
	for i := range dp {
		dp[i] = INF
		cut[i] = -1
	}
	dp[0] = 0
	for i := 1; i <= n; i++ {
		for j := 0; j < i; j++ {
			if pal[j][i-1] && dp[j]+1 < dp[i] {
				dp[i] = dp[j] + 1
				cut[i] = j
			}
		}
	}

	res := []string{}
	for i := n; i > 0; i = cut[i] {
		res = append(res, s[cut[i]:i])
	}
	// reverse
	for l, r := 0, len(res)-1; l < r; l, r = l+1, r-1 {
		res[l], res[r] = res[r], res[l]
	}
	return res
}

func main() {
	fmt.Println(minPalindromePartition("racecarannakayak"))
	fmt.Println(minPalindromePartition("abc"))
}
