// Count ways to roll N dice (faces each) summing to total via DP.
// Time O(N*faces*total), Space O(total).
package main

import "fmt"

func throwDice(N, faces, total int) int64 {
	dp := make([]int64, total+1)
	dp[0] = 1
	for d := 0; d < N; d++ {
		nxt := make([]int64, total+1)
		for s := 0; s <= total; s++ {
			if dp[s] != 0 {
				for f := 1; f <= faces && s+f <= total; f++ {
					nxt[s+f] += dp[s]
				}
			}
		}
		dp = nxt
	}
	return dp[total]
}

func main() {
	fmt.Println(throwDice(3, 6, 7))
}
