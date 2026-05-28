// Day 1580: Largest divisible subset (every pair mutually divisible).
// Sort, then LIS-style DP: dp[i] = longest chain ending at i where a[i] % a[j] == 0.
// Time: O(n^2); Space: O(n).
package main

import (
	"fmt"
	"sort"
)

func largestDivisible(a []int) []int {
	sort.Ints(a)
	n := len(a)
	if n == 0 {
		return []int{}
	}
	dp := make([]int, n)
	prev := make([]int, n)
	best := 0
	for i := range dp {
		dp[i] = 1
		prev[i] = -1
	}
	for i := 0; i < n; i++ {
		for j := 0; j < i; j++ {
			if a[i]%a[j] == 0 && dp[j]+1 > dp[i] {
				dp[i] = dp[j] + 1
				prev[i] = j
			}
		}
		if dp[i] > dp[best] {
			best = i
		}
	}
	var res []int
	for i := best; i != -1; i = prev[i] {
		res = append(res, a[i])
	}
	for l, r := 0, len(res)-1; l < r; l, r = l+1, r-1 {
		res[l], res[r] = res[r], res[l]
	}
	return res
}

func main() {
	fmt.Println(largestDivisible([]int{3, 5, 10, 20, 21})) // [5 10 20]
}
