// Day 666: Minimize difference of two subset sums. Subset-sum DP over reachable sums up to total/2,
// pick reachable sum closest to total/2, reconstruct one subset. Time O(n*sum), Space O(n*sum).
package main

import (
	"fmt"
	"strings"
)

func main() {
	a := []int{5, 10, 15, 20, 25}
	n, total := len(a), 0
	for _, x := range a {
		total += x
	}
	half := total / 2
	dp := make([][]bool, n+1)
	for i := range dp {
		dp[i] = make([]bool, half+1)
		dp[i][0] = true
	}
	for i := 1; i <= n; i++ {
		for s := 0; s <= half; s++ {
			dp[i][s] = dp[i-1][s] || (s >= a[i-1] && dp[i-1][s-a[i-1]])
		}
	}
	best := 0
	for s := half; s >= 0; s-- {
		if dp[n][s] {
			best = s
			break
		}
	}
	var A, B []int
	s := best
	for i := n; i >= 1; i-- {
		if s >= a[i-1] && dp[i-1][s-a[i-1]] {
			A = append(A, a[i-1])
			s -= a[i-1]
		} else {
			B = append(B, a[i-1])
		}
	}
	fmt.Printf("{%s} and {%s}, difference of %d\n", join(A), join(B), total-2*best)
}

func join(v []int) string {
	s := make([]string, len(v))
	for i, x := range v {
		s[i] = fmt.Sprint(x)
	}
	return strings.Join(s, ", ")
}
