// Partition equal subset sum: if total odd -> false, else subset-sum DP for sum/2.
// Time O(n*sum), Space O(sum).
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
		for j := target; j >= x; j-- {
			if dp[j-x] {
				dp[j] = true
			}
		}
	}
	return dp[target]
}

func main() {
	a := []int{15, 5, 20, 10, 35, 15, 10}
	b := []int{15, 5, 20, 10, 35}
	fmt.Println(canPartition(a))
	fmt.Println(canPartition(b))
}
