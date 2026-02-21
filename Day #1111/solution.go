// Day 1111 - Lazy bartender (minimum set cover)
// Approach: exact set cover via DP over bitmask of covered customers.
// Time: O(D * 2^C), Space: O(2^C).
package main

import (
	"fmt"
	"sort"
)

func minDrinks(preferences map[int][]int) int {
	customers := make([]int, 0, len(preferences))
	for c := range preferences {
		customers = append(customers, c)
	}
	sort.Ints(customers)
	n := len(customers)
	cidx := make(map[int]int)
	for i, c := range customers {
		cidx[c] = i
	}

	drinkMask := make(map[int]int)
	for c, drinks := range preferences {
		for _, d := range drinks {
			drinkMask[d] |= 1 << cidx[c]
		}
	}

	full := (1 << n) - 1
	const INF = int(^uint(0) >> 1)
	dp := make([]int, 1<<n)
	for i := range dp {
		dp[i] = INF
	}
	dp[0] = 0
	for s := 0; s < (1 << n); s++ {
		if dp[s] == INF {
			continue
		}
		for _, m := range drinkMask {
			ns := s | m
			if dp[ns] > dp[s]+1 {
				dp[ns] = dp[s] + 1
			}
		}
	}
	return dp[full]
}

func main() {
	preferences := map[int][]int{
		0: {0, 1, 3, 6},
		1: {1, 4, 7},
		2: {2, 4, 7, 5},
		3: {3, 2, 5},
		4: {5, 8},
	}
	fmt.Println(minDrinks(preferences)) // 2
}
