// Day 1123 - Largest divisible subset
// Sort, LIS-style DP where j extends i if a[i] % a[j] == 0; reconstruct via
// parent pointers. Time: O(n^2), Space: O(n).
package main

import (
	"fmt"
	"sort"
)

func largestDivisibleSubset(nums []int) []int {
	if len(nums) == 0 {
		return []int{}
	}
	sorted := append([]int(nil), nums...)
	sort.Ints(sorted)
	n := len(sorted)
	dp := make([]int, n)
	parent := make([]int, n)
	best := 0
	for i := range dp {
		dp[i] = 1
		parent[i] = -1
	}
	for i := 0; i < n; i++ {
		for j := 0; j < i; j++ {
			if sorted[i]%sorted[j] == 0 && dp[j]+1 > dp[i] {
				dp[i] = dp[j] + 1
				parent[i] = j
			}
		}
		if dp[i] > dp[best] {
			best = i
		}
	}
	var res []int
	for k := best; k != -1; k = parent[k] {
		res = append([]int{sorted[k]}, res...)
	}
	return res
}

func main() {
	fmt.Println(largestDivisibleSubset([]int{3, 5, 10, 20, 21})) // [5 10 20]
	fmt.Println(largestDivisibleSubset([]int{1, 3, 6, 24}))      // [1 3 6 24]
}
