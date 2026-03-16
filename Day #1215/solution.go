// Day 1215: Min steps to reduce N to 1 (decrement, or replace by larger factor).
// DP: dp[i] = 1 + min(dp[i-1], dp[i/d] for divisors d). Time O(N sqrt N), Space O(N).
package main

import "fmt"

func minSteps(n int) int {
	dp := make([]int, n+1)
	for i := 2; i <= n; i++ {
		dp[i] = dp[i-1] + 1
		for d := 2; d*d <= i; d++ {
			if i%d == 0 && dp[i/d]+1 < dp[i] {
				dp[i] = dp[i/d] + 1
			}
		}
	}
	return dp[n]
}

func main() {
	fmt.Println(minSteps(100)) // 5
}
