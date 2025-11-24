// Road-trip playlist: DP (LeetCode 920). dp[i][j]=playlists of length i with j distinct songs.
// dp[i][j]=dp[i-1][j-1]*(M-(j-1)) + dp[i-1][j]*max(j-B,0). Time O(N*M), space O(M).
package main

import "fmt"

const MOD = 1000000007

func countPlaylists(N, M, B int) int64 {
	prev := make([]int64, M+1)
	prev[0] = 1
	for i := 1; i <= N; i++ {
		cur := make([]int64, M+1)
		for j := 1; j <= M; j++ {
			cur[j] = prev[j-1] * int64(M-(j-1)) % MOD
			mult := j - B
			if mult < 0 {
				mult = 0
			}
			cur[j] = (cur[j] + prev[j]*int64(mult)) % MOD
		}
		prev = cur
	}
	return prev[M]
}

func main() {
	N, M, B := 3, 3, 1
	fmt.Println(countPlaylists(N, M, B))
}
