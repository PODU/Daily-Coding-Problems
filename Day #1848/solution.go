// Day 1848: Longest common subsequence of three strings via 3D dynamic programming.
// Time O(L1*L2*L3), Space O(L1*L2*L3).
package main

import "fmt"

func lcs3(a, b, c string) int {
	la, lb, lc := len(a), len(b), len(c)
	dp := make([][][]int, la+1)
	for i := range dp {
		dp[i] = make([][]int, lb+1)
		for j := range dp[i] {
			dp[i][j] = make([]int, lc+1)
		}
	}
	max3 := func(x, y, z int) int {
		m := x
		if y > m {
			m = y
		}
		if z > m {
			m = z
		}
		return m
	}
	for i := 1; i <= la; i++ {
		for j := 1; j <= lb; j++ {
			for k := 1; k <= lc; k++ {
				if a[i-1] == b[j-1] && b[j-1] == c[k-1] {
					dp[i][j][k] = dp[i-1][j-1][k-1] + 1
				} else {
					dp[i][j][k] = max3(dp[i-1][j][k], dp[i][j-1][k], dp[i][j][k-1])
				}
			}
		}
	}
	return dp[la][lb][lc]
}

func main() {
	fmt.Println(lcs3("epidemiologist", "refrigeration",
		"supercalifragilisticexpialodocious")) // 5
}
