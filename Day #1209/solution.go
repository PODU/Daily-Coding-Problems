// Day 1209: Largest divisible subset.
// Sort, dp[i]=longest chain ending at i with parent pointers. Time O(n^2), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func largestDivisibleSubset(a []int) []int {
	sort.Ints(a)
	n := len(a)
	if n == 0 {
		return nil
	}
	dp := make([]int, n)
	par := make([]int, n)
	best := 0
	for i := range dp {
		dp[i] = 1
		par[i] = -1
	}
	for i := 0; i < n; i++ {
		for j := 0; j < i; j++ {
			if a[i]%a[j] == 0 && dp[j]+1 > dp[i] {
				dp[i] = dp[j] + 1
				par[i] = j
			}
		}
		if dp[i] > dp[best] {
			best = i
		}
	}
	var res []int
	for i := best; i != -1; i = par[i] {
		res = append(res, a[i])
	}
	for l, r := 0, len(res)-1; l < r; l, r = l+1, r-1 {
		res[l], res[r] = res[r], res[l]
	}
	return res
}

func main() {
	fmt.Println(largestDivisibleSubset([]int{3, 5, 10, 20, 21})) // [5 10 20]
}
