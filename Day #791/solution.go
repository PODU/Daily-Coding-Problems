// throw_dice: DP over dice, dp[s] = ways to reach sum s. Rolling array.
// Time O(N*total*faces), Space O(total).
package main

import "fmt"

func throwDice(N, faces, total int) int64 {
	dp := make([]int64, total+1)
	dp[0] = 1
	for i := 0; i < N; i++ {
		ndp := make([]int64, total+1)
		for s := 0; s <= total; s++ {
			if dp[s] == 0 {
				continue
			}
			for f := 1; f <= faces && s+f <= total; f++ {
				ndp[s+f] += dp[s]
			}
		}
		dp = ndp
	}
	return dp[total]
}

func main() {
	fmt.Println(throwDice(3, 6, 7))
}
