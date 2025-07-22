// Staircase ways with step set X: dp[i] = sum dp[i-x] for x in X.
// Time: O(N*|X|), Space: O(N).
package main

import "fmt"

func staircase(n int, X []int) int64 {
	dp := make([]int64, n+1)
	dp[0] = 1
	for i := 1; i <= n; i++ {
		for _, x := range X {
			if i-x >= 0 {
				dp[i] += dp[i-x]
			}
		}
	}
	return dp[n]
}

func main() {
	fmt.Println(staircase(4, []int{1, 2})) // 5
}
