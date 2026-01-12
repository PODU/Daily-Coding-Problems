// Min steps to 1: dp[i] = 1 + min(dp[i-1], dp[i/a] over factors a). Time O(N*sqrt N), Space O(N).
package main

import "fmt"

func minSteps(n int) int {
	dp := make([]int, n+1)
	for i := 2; i <= n; i++ {
		dp[i] = dp[i-1] + 1
		for a := 2; a*a <= i; a++ {
			if i%a == 0 {
				larger := i / a
				if 1+dp[larger] < dp[i] {
					dp[i] = 1 + dp[larger]
				}
			}
		}
	}
	return dp[n]
}

func main() {
	fmt.Println(minSteps(100))
}
