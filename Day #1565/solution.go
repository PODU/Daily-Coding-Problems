// Subset-sum DP: partition into equal halves iff target=sum/2 reachable. Time O(n*sum), Space O(sum).
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
	nums := []int{15, 5, 20, 10, 35, 15, 10}
	if canPartition(nums) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
