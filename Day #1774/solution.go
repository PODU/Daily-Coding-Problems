// Day 1774: Fewest steps to reduce N to 1 (decrement by 1, or jump to the larger
// factor of any factorization). DP over 1..N, trying every divisor. O(N*sqrt N) time, O(N) space.
package main

import "fmt"

func minSteps(N int) int {
	dp := make([]int, N+1)
	for i := 2; i <= N; i++ {
		dp[i] = dp[i-1] + 1 // decrement step
		for a := 2; a*a <= i; a++ {
			if i%a == 0 && dp[i/a]+1 < dp[i] {
				dp[i] = dp[i/a] + 1 // jump to larger factor i/a
			}
		}
	}
	return dp[N]
}

func main() {
	fmt.Println(minSteps(100)) // 100->10->9->3->2->1 = 5
}
