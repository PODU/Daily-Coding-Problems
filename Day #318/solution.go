// Count playlists of length N from M songs, each used >=1, gap >=B between repeats.
// DP over length x distinct songs (LeetCode 920). Time O(N*M), Space O(N*M).
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
			g := j - B
			if g < 0 {
				g = 0
			}
			dp[i][j] = (dp[i][j] + dp[i-1][j]*int64(g)) % MOD
		}
	}
	return dp[N][M]
}

func main() {
	N, M, B := 3, 3, 1
	fmt.Printf("Number of valid playlists (N=%d, M=%d, B=%d) = %d\n", N, M, B, numPlaylists(N, M, B))
}
