// Day 1033: Minimum subset-sum difference (partition into two subsets).
// Boolean subset-sum DP over reachable sums up to total/2; answer total-2*best. O(n*sum) time, O(sum) space.
package main

import "fmt"

func minDiff(a []int) int {
	total := 0
	for _, x := range a {
		total += x
	}
	half := total / 2
	dp := make([]bool, half+1)
	dp[0] = true
	for _, x := range a {
		for s := half; s >= x; s-- {
			if dp[s-x] {
				dp[s] = true
			}
		}
	}
	for s := half; s >= 0; s-- {
		if dp[s] {
			return total - 2*s
		}
	}
	return total
}

func main() {
	fmt.Println(minDiff([]int{5, 10, 15, 20, 25}))
}
