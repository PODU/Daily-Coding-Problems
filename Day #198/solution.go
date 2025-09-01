// Day 198: Largest divisible subset.
// Sort, then DP: dp[i] = longest chain ending at i (a[j] | a[i]); track parent for reconstruction.
// Time: O(n^2), Space: O(n).
package main

import (
	"fmt"
	"sort"
)

func largestDivisibleSubset(arr []int) []int {
	a := append([]int(nil), arr...)
	sort.Ints(a)
	n := len(a)
	if n == 0 {
		return []int{}
	}
	dp := make([]int, n)
	parent := make([]int, n)
	best := 0
	for i := range dp {
		dp[i] = 1
		parent[i] = -1
	}
	for i := 0; i < n; i++ {
		for j := 0; j < i; j++ {
			if a[i]%a[j] == 0 && dp[j]+1 > dp[i] {
				dp[i] = dp[j] + 1
				parent[i] = j
			}
		}
		if dp[i] > dp[best] {
			best = i
		}
	}
	var res []int
	for i := best; i != -1; i = parent[i] {
		res = append(res, a[i])
	}
	for l, r := 0, len(res)-1; l < r; l, r = l+1, r-1 {
		res[l], res[r] = res[r], res[l]
	}
	return res
}

func main() {
	fmt.Println(largestDivisibleSubset([]int{3, 5, 10, 20, 21})) // [5 10 20]
	fmt.Println(largestDivisibleSubset([]int{1, 3, 6, 24}))      // [1 3 6 24]
}
