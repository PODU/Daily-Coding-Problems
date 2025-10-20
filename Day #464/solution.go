// Day 464: Largest divisible subset.
// Approach: sort, then LIS-style DP where dp[i]=longest chain ending at i;
// j divides i means a[i]%a[j]==0. Reconstruct via parent pointers.
// Time: O(n^2), Space: O(n).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func largestDivisibleSubset(nums []int) []int {
	if len(nums) == 0 {
		return []int{}
	}
	sort.Ints(nums)
	n := len(nums)
	dp := make([]int, n)
	parent := make([]int, n)
	best := 0
	for i := range nums {
		dp[i] = 1
		parent[i] = -1
	}
	for i := 0; i < n; i++ {
		for j := 0; j < i; j++ {
			if nums[i]%nums[j] == 0 && dp[j]+1 > dp[i] {
				dp[i] = dp[j] + 1
				parent[i] = j
			}
		}
		if dp[i] > dp[best] {
			best = i
		}
	}
	var res []int
	for i := best; i >= 0; i = parent[i] {
		res = append(res, nums[i])
	}
	for l, r := 0, len(res)-1; l < r; l, r = l+1, r-1 {
		res[l], res[r] = res[r], res[l]
	}
	return res
}

func format(v []int) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = fmt.Sprintf("%d", x)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(format(largestDivisibleSubset([]int{3, 5, 10, 20, 21})))
	fmt.Println(format(largestDivisibleSubset([]int{1, 3, 6, 24})))
}
