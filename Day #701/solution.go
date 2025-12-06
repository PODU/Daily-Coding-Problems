// Day 701: Minimum drinks to satisfy every customer (minimum set cover).
// Approach: each drink -> bitmask of customers it satisfies; DP over customer
// masks for fewest drinks covering all. Time O(2^C * D), Space O(2^C).
package main

import (
	"fmt"
	"math"
	"sort"
)

func minDrinks(prefs map[int][]int) int {
	custs := make([]int, 0, len(prefs))
	for c := range prefs {
		custs = append(custs, c)
	}
	sort.Ints(custs)
	idx := make(map[int]int)
	for i, c := range custs {
		idx[c] = i
	}
	C := len(custs)
	drinkMask := make(map[int]int)
	for cust, drinks := range prefs {
		for _, d := range drinks {
			drinkMask[d] |= 1 << idx[cust]
		}
	}
	full := (1 << C) - 1
	dp := make([]int, full+1)
	for i := range dp {
		dp[i] = math.MaxInt32
	}
	dp[0] = 0
	for mask := 0; mask <= full; mask++ {
		if dp[mask] == math.MaxInt32 {
			continue
		}
		for _, dm := range drinkMask {
			nm := mask | dm
			if dp[mask]+1 < dp[nm] {
				dp[nm] = dp[mask] + 1
			}
		}
	}
	return dp[full]
}

func main() {
	prefs := map[int][]int{
		0: {0, 1, 3, 6},
		1: {1, 4, 7},
		2: {2, 4, 7, 5},
		3: {3, 2, 5},
		4: {5, 8},
	}
	fmt.Println(minDrinks(prefs)) // 2
}
