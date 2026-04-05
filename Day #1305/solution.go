// Day 1305: Longest common subsequence of three strings.
// 3D DP over prefixes. O(|a||b||c|) time, O(|a||b||c|) space.
package main

import "fmt"

func max3(a, b, c int) int {
	m := a
	if b > m {
		m = b
	}
	if c > m {
		m = c
	}
	return m
}

func lcs3(a, b, c string) int {
	n, m, p := len(a), len(b), len(c)
	dp := make([][][]int, n+1)
	for i := range dp {
		dp[i] = make([][]int, m+1)
		for j := range dp[i] {
			dp[i][j] = make([]int, p+1)
		}
	}
	for i := 1; i <= n; i++ {
		for j := 1; j <= m; j++ {
			for k := 1; k <= p; k++ {
				if a[i-1] == b[j-1] && b[j-1] == c[k-1] {
					dp[i][j][k] = dp[i-1][j-1][k-1] + 1
				} else {
					dp[i][j][k] = max3(dp[i-1][j][k], dp[i][j-1][k], dp[i][j][k-1])
				}
			}
		}
	}
	return dp[n][m][p]
}

func main() {
	fmt.Println(lcs3("epidemiologist", "refrigeration",
		"supercalifragilisticexpialodocious")) // 5
}
