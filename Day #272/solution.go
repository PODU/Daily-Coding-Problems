// Day 272: throw_dice(N, faces, total) -> number of ways to reach total.
// 1-D DP over rolling sums. Time O(N*total*faces), Space O(total).
package main

import "fmt"

func throwDice(N, faces, total int) int64 {
	dp := make([]int64, total+1)
	dp[0] = 1
	for d := 0; d < N; d++ {
		ndp := make([]int64, total+1)
		for t := 0; t <= total; t++ {
			if dp[t] == 0 {
				continue
			}
			for f := 1; f <= faces && t+f <= total; f++ {
				ndp[t+f] += dp[t]
			}
		}
		dp = ndp
	}
	return dp[total]
}

func main() {
	fmt.Println(throwDice(3, 6, 7)) // 15
}
