// Day 186: Minimum subset-sum difference (partition problem).
// Subset-sum DP over total, pick best <= total/2, reconstruct. Time O(n*S), Space O(n*S).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func solve(a []int) {
	n, tot := len(a), 0
	for _, x := range a {
		tot += x
	}
	dp := make([][]bool, n+1)
	for i := range dp {
		dp[i] = make([]bool, tot+1)
	}
	dp[0][0] = true
	for i := 1; i <= n; i++ {
		for s := 0; s <= tot; s++ {
			dp[i][s] = dp[i-1][s] || (s >= a[i-1] && dp[i-1][s-a[i-1]])
		}
	}
	best := 0
	for s := tot / 2; s >= 0; s-- {
		if dp[n][s] {
			best = s
			break
		}
	}
	var sub, other []int
	s := best
	for i := n; i >= 1; i-- {
		if s >= a[i-1] && dp[i-1][s-a[i-1]] {
			sub = append(sub, a[i-1])
			s -= a[i-1]
		} else {
			other = append(other, a[i-1])
		}
	}
	rev := func(v []int) { for i, j := 0, len(v)-1; i < j; i, j = i+1, j-1 { v[i], v[j] = v[j], v[i] } }
	rev(sub)
	rev(other)
	f := func(v []int) string {
		parts := make([]string, len(v))
		for i, x := range v {
			parts[i] = strconv.Itoa(x)
		}
		return "{" + strings.Join(parts, ", ") + "}"
	}
	fmt.Printf("%s and %s\n", f(sub), f(other))
}

func main() {
	solve([]int{5, 10, 15, 20, 25})
}
