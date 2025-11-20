// Day 632: Deduce coin denominations from a "ways to make change" array.
// Approach: reverse coin-change DP. If ways[i] exceeds count reachable with
// coins found so far, i is itself a denomination.
// Time: O(N * D), Space: O(N).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func findDenominations(ways []int64) []int {
	n := len(ways)
	dp := make([]int64, n)
	dp[0] = 1
	coins := []int{}
	for i := 1; i < n; i++ {
		if dp[i] < ways[i] {
			coins = append(coins, i)
			for j := i; j < n; j++ {
				dp[j] += dp[j-i]
			}
		}
	}
	return coins
}

func main() {
	ways := []int64{1, 0, 1, 1, 2}
	coins := findDenominations(ways)
	strs := make([]string, len(coins))
	for i, c := range coins {
		strs[i] = strconv.Itoa(c)
	}
	if len(strs) > 1 {
		fmt.Println(strings.Join(strs[:len(strs)-1], ", ") + ", and " + strs[len(strs)-1])
	} else {
		fmt.Println(strings.Join(strs, ", "))
	}
}
