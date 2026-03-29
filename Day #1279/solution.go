// Day 1279: Lazy bartender = minimum set cover over customers.
// DP over customer bitmask. Time O(2^C * D), Space O(2^C). C=#customers, D=#drinks.
package main

import "fmt"

func minDrinks(prefs [][]int) int {
	C := len(prefs)
	drinkMask := map[int]int{}
	for cust := 0; cust < C; cust++ {
		for _, d := range prefs[cust] {
			drinkMask[d] |= 1 << cust
		}
	}
	full := (1 << C) - 1
	const INF = 1 << 30
	dp := make([]int, 1<<C)
	for i := range dp {
		dp[i] = INF
	}
	dp[0] = 0
	masks := make([]int, 0, len(drinkMask))
	for _, m := range drinkMask {
		masks = append(masks, m)
	}
	for mask := 0; mask <= full; mask++ {
		if dp[mask] == INF {
			continue
		}
		for _, dm := range masks {
			nm := mask | dm
			if dp[mask]+1 < dp[nm] {
				dp[nm] = dp[mask] + 1
			}
		}
	}
	return dp[full]
}

func main() {
	prefs := [][]int{{0, 1, 3, 6}, {1, 4, 7}, {2, 4, 7, 5}, {3, 2, 5}, {5, 8}}
	fmt.Println(minDrinks(prefs)) // 2
}
