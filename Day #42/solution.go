// Subset Sum: boolean DP dp[i][j] = can make j with first i items, then backtrack.
// Found subset is sorted descending for a deterministic [12, 9, 2, 1] output.
// Time O(n*k), Space O(n*k).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func subsetSum(S []int, k int) []int {
	n := len(S)
	dp := make([][]bool, n+1)
	for i := range dp {
		dp[i] = make([]bool, k+1)
		dp[i][0] = true
	}
	for i := 1; i <= n; i++ {
		for j := 1; j <= k; j++ {
			dp[i][j] = dp[i-1][j]
			if j >= S[i-1] && dp[i-1][j-S[i-1]] {
				dp[i][j] = true
			}
		}
	}
	if !dp[n][k] {
		return nil
	}
	res := []int{}
	j := k
	for i := n; i >= 1; i-- {
		if dp[i-1][j] { // item i-1 not needed
			continue
		}
		res = append(res, S[i-1]) // item i-1 must be included
		j -= S[i-1]
	}
	sort.Sort(sort.Reverse(sort.IntSlice(res)))
	return res
}

func fmtList(v []int) string {
	if v == nil {
		return "null"
	}
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = strconv.Itoa(x)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	S := []int{12, 1, 61, 5, 9, 2}
	fmt.Println(fmtList(subsetSum(S, 24)))
}
