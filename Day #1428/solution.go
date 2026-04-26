// Day 1428: Partition array into two subsets minimizing sum difference.
// Approach: subset-sum DP over half the total; reconstruct one subset.
// Time: O(n * sum), Space: O(n * sum).
package main

import "fmt"

func main() {
	a := []int{5, 10, 15, 20, 25}
	n := len(a)
	total := 0
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
			dp[i][s] = dp[i-1][s]
			if s >= a[i-1] && dp[i-1][s-a[i-1]] {
				dp[i][s] = true
			}
		}
	}

	best := 0
	for s := half; s >= 0; s-- {
		if dp[n][s] {
			best = s
			break
		}
	}

	subset := []int{}
	s := best
	for i := n; i >= 1; i-- {
		if s >= a[i-1] && dp[i-1][s-a[i-1]] {
			subset = append(subset, a[i-1])
			s -= a[i-1]
		}
	}

	diff := total - 2*best
	fmt.Println("Difference:", diff) // Difference: 5
	fmt.Println("Subset:", subset)
}
