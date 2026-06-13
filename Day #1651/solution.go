// Day 1651: Count valid playlists of length N from M songs, each used >=1 time,
// with a buffer of B songs between repeats.
// DP: dp[i][j] = #playlists of length i using exactly j distinct songs.
// Time O(N*M), Space O(N*M).
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
			rem := j - B
			if rem < 0 {
				rem = 0
			}
			dp[i][j] = (dp[i][j] + dp[i-1][j]*int64(rem)) % MOD
		}
	}
	return dp[N][M]
}

func main() {
	// N=3, M=2, B=0 -> 6
	fmt.Println(numPlaylists(3, 2, 0))
}
