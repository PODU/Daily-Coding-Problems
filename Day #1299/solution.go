// Day 1299: Count ordered ways to climb N stairs using step sizes from set X.
// DP: dp[i] = sum(dp[i-x]) for x in X. O(N*|X|) time, O(N) space.
package main

import "fmt"

func climbWays(n int, steps []int) int64 {
	dp := make([]int64, n+1)
	dp[0] = 1
	for i := 1; i <= n; i++ {
		for _, x := range steps {
			if i-x >= 0 {
				dp[i] += dp[i-x]
			}
		}
	}
	return dp[n]
}

func main() {
	fmt.Println(climbWays(4, []int{1, 2}))     // 5
	fmt.Println(climbWays(10, []int{1, 3, 5})) // generalized
}
