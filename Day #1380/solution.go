// Subset sum returning an actual subset via DP + backtracking reconstruction.
// Time O(n*k), Space O(n*k). Returns nil ("null") if no subset sums to k.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func subsetSum(s []int, k int) []int {
	n := len(s)
	dp := make([][]bool, n+1)
	for i := range dp {
		dp[i] = make([]bool, k+1)
		dp[i][0] = true
	}
	for i := 1; i <= n; i++ {
		for j := 0; j <= k; j++ {
			dp[i][j] = dp[i-1][j]
			if j >= s[i-1] && dp[i-1][j-s[i-1]] {
				dp[i][j] = true
			}
		}
	}
	if !dp[n][k] {
		return nil
	}
	var res []int
	j := k
	for i := n; i >= 1; i-- {
		if !dp[i-1][j] {
			res = append(res, s[i-1])
			j -= s[i-1]
		}
	}
	for l, r := 0, len(res)-1; l < r; l, r = l+1, r-1 {
		res[l], res[r] = res[r], res[l]
	}
	return res
}

func main() {
	res := subsetSum([]int{12, 1, 61, 5, 9, 2}, 24)
	if res == nil {
		fmt.Println("null")
		return
	}
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
