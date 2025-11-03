// Subset-sum DP with reconstruction. O(n*k) time, O(n*k) space. Result sorted desc.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func subsetSum(s []int, k int) ([]int, bool) {
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
		return nil, false
	}
	var res []int
	j := k
	for i := n; i >= 1; i-- {
		if !dp[i-1][j] {
			res = append(res, s[i-1])
			j -= s[i-1]
		}
	}
	sort.Sort(sort.Reverse(sort.IntSlice(res)))
	return res, true
}

func printRes(res []int, ok bool) {
	if !ok {
		fmt.Println("null")
		return
	}
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}

func main() {
	s := []int{12, 1, 61, 5, 9, 2}
	printRes(subsetSum(s, 24))
	printRes(subsetSum(s, 1000))
}
