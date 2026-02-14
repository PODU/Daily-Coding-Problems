// Subset sum DP: dp[i][s] = reachable using first i items; reconstruct path. O(n*k) time/space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func subsetSum(items []int, k int) []int {
	n := len(items)
	dp := make([][]bool, n+1)
	for i := range dp {
		dp[i] = make([]bool, k+1)
	}
	dp[0][0] = true
	for i := 1; i <= n; i++ {
		for s := 0; s <= k; s++ {
			dp[i][s] = dp[i-1][s]
			if !dp[i][s] && s >= items[i-1] {
				dp[i][s] = dp[i-1][s-items[i-1]]
			}
		}
	}
	if !dp[n][k] {
		return nil
	}
	var result []int
	s := k
	for i := n; i >= 1; i-- {
		if !dp[i-1][s] {
			result = append(result, items[i-1])
			s -= items[i-1]
		}
	}
	for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
		result[i], result[j] = result[j], result[i]
	}
	return result
}

func formatSlice(s []int) string {
	parts := make([]string, len(s))
	for i, v := range s {
		parts[i] = strconv.Itoa(v)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	items := []int{12, 1, 61, 5, 9, 2}
	res := subsetSum(items, 24)
	fmt.Printf("Subset: %s\n", formatSlice(res))
	sum := 0
	for _, v := range res {
		sum += v
	}
	fmt.Printf("Sum: %d\n", sum)

	res2 := subsetSum(items, 1000)
	if res2 == nil {
		fmt.Println("null")
	}
}
