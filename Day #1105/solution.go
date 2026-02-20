// Day 1105: Partition multiset into two equal-sum subsets (subset-sum DP).
// If total odd -> false; else can we reach total/2. Time: O(N*sum). Space: O(sum).
package main

import "fmt"

func canPartition(nums []int) bool {
	total := 0
	for _, x := range nums {
		total += x
	}
	if total%2 == 1 {
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
	fmt.Println(canPartition([]int{15, 5, 20, 10, 35, 15, 10})) // true
	fmt.Println(canPartition([]int{15, 5, 20, 10, 35}))         // false
}
