// Min steps to reduce N to 1 (N-1 or replace with max factor): DP.
// Time O(N*sqrt(N)), Space O(N).
package main

import "fmt"

func minSteps(N int) int {
	dp := make([]int, N+1)
	for n := 2; n <= N; n++ {
		dp[n] = dp[n-1] + 1
		for a := 2; a*a <= n; a++ {
			if n%a == 0 {
				b := n / a // b >= a, max(a,b)=b
				if dp[b]+1 < dp[n] {
					dp[n] = dp[b] + 1
				}
			}
		}
	}
	return dp[N]
}

func main() {
	fmt.Println(minSteps(100))
}
