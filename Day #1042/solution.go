// LCS of three strings via 3D DP dp[i][j][k]. Time O(n1*n2*n3), Space O(n1*n2*n3).
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
	n1, n2, n3 := len(a), len(b), len(c)
	dp := make([][][]int, n1+1)
	for i := range dp {
		dp[i] = make([][]int, n2+1)
		for j := range dp[i] {
			dp[i][j] = make([]int, n3+1)
		}
	}
	for i := 1; i <= n1; i++ {
		for j := 1; j <= n2; j++ {
			for k := 1; k <= n3; k++ {
				if a[i-1] == b[j-1] && b[j-1] == c[k-1] {
					dp[i][j][k] = dp[i-1][j-1][k-1] + 1
				} else {
					dp[i][j][k] = max3(dp[i-1][j][k], dp[i][j-1][k], dp[i][j][k-1])
				}
			}
		}
	}
	return dp[n1][n2][n3]
}

func main() {
	fmt.Println(lcs3("epidemiologist", "refrigeration",
		"supercalifragilisticexpialodocious"))
}
