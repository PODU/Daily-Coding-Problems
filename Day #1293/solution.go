// Day 1293: Count valid playlists of length N from M songs, each used >=1, repeats >=B apart.
// DP: dp[i][j]=playlists of len i with j distinct songs. O(N*M) time, O(N*M) space (mod 1e9+7).
package main

import "fmt"

const MOD = 1000000007

func numPlaylists(N, M, B int) int64 {
	dp := make([][]int64, N+1)
	for i := range dp {
		dp[i] = make([]int64, M+1)
	}
	dp[0][0] = 1
	for i := 1; i <= N; i++ {
		for j := 1; j <= M; j++ {
			dp[i][j] = dp[i-1][j-1] * int64(M-(j-1)) % MOD
			reuse := j - B
			if reuse < 0 {
				reuse = 0
			}
			dp[i][j] = (dp[i][j] + dp[i-1][j]*int64(reuse)) % MOD
		}
	}
	return dp[N][M]
}

func main() {
	fmt.Println(numPlaylists(3, 3, 1)) // 6
}
