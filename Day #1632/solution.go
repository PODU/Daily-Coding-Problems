// Min subset-sum difference: 0/1 subset-sum DP over reachable sums, pick best<=total/2; backtrack subset. O(n*total) time/space.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func format(v []int) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = strconv.Itoa(x)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	a := []int{5, 10, 15, 20, 25}
	n := len(a)
	total := 0
	for _, x := range a {
		total += x
	}

	// dp[i][s] = sum s reachable using first i items
	dp := make([][]bool, n+1)
	for i := range dp {
		dp[i] = make([]bool, total+1)
	}
	dp[0][0] = true
	for i := 1; i <= n; i++ {
		for s := 0; s <= total; s++ {
			dp[i][s] = dp[i-1][s]
			if s >= a[i-1] && dp[i-1][s-a[i-1]] {
				dp[i][s] = true
			}
		}
	}

	best := 0
	for s := total / 2; s >= 0; s-- {
		if dp[n][s] {
			best = s
			break
		}
	}

	// Backtrack from last item to first to recover subset A
	A := []int{}
	used := make([]bool, n)
	s := best
	for i := n; i >= 1; i-- {
		if s >= a[i-1] && dp[i-1][s-a[i-1]] {
			A = append(A, a[i-1])
			used[i-1] = true
			s -= a[i-1]
		}
	}
	sort.Ints(A)

	B := []int{}
	for i := 0; i < n; i++ {
		if !used[i] {
			B = append(B, a[i])
		}
	}

	fmt.Println("Minimum difference:", total-2*best)
	fmt.Println("Subset A:", format(A))
	fmt.Println("Subset B:", format(B))
}
