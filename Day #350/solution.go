// Min perfect squares summing to N via DP, then greedy-largest reconstruction.
// Time O(N*sqrt N), Space O(N).
package main

import (
	"fmt"
	"math"
	"strings"
)

func solve(n int) string {
	dp := make([]int, n+1)
	for i := 1; i <= n; i++ {
		dp[i] = math.MaxInt32
		for s := 1; s*s <= i; s++ {
			if dp[i-s*s]+1 < dp[i] {
				dp[i] = dp[i-s*s] + 1
			}
		}
	}

	// Reconstruct: greedily pick largest s with dp[i - s*s] == dp[i] - 1.
	var squares []int
	i := n
	for i > 0 {
		for s := int(math.Sqrt(float64(i))); s >= 1; s-- {
			if s*s <= i && dp[i-s*s] == dp[i]-1 {
				squares = append(squares, s*s)
				i -= s * s
				break
			}
		}
	}

	parts := make([]string, len(squares))
	for k, v := range squares {
		parts[k] = fmt.Sprintf("%d", v)
	}
	return fmt.Sprintf("%d (%s)", dp[n], strings.Join(parts, " + "))
}

func main() {
	for _, n := range []int{4, 17, 18} {
		fmt.Println(solve(n))
	}
}
