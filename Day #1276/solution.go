// Day 1276: Partition a multiset into two subsets of equal sum.
// Subset-sum DP (can we reach totalSum/2?). Time O(n*S), Space O(S).
package main

import "fmt"

func canPartition(nums []int) bool {
	total := 0
	for _, x := range nums {
		total += x
	}
	if total%2 != 0 {
		return false
	}
	target := total / 2
	dp := make([]bool, target+1)
	dp[0] = true
	for _, x := range nums {
		for s := target; s >= x; s-- {
			if dp[s-x] {
				dp[s] = true
			}
		}
	}
	return dp[target]
}

func main() {
	fmt.Println(canPartition([]int{15, 5, 20, 10, 35, 15, 10}))
	fmt.Println(canPartition([]int{15, 5, 20, 10, 35}))
}
