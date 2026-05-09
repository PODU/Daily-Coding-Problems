// Day 1496: Min cost to paint N houses with K colors, no two adjacent same color.
// Approach: DP tracking previous row's min & 2nd-min (+min index). Time O(N*K), Space O(1).
package main

import (
	"fmt"
	"math"
)

func minCost(costs [][]int) int {
	if len(costs) == 0 {
		return 0
	}
	prevMin1, prevMin2, prevIdx := 0, 0, -1
	for _, row := range costs {
		curMin1, curMin2, curIdx := math.MaxInt64, math.MaxInt64, -1
		for k, base := range row {
			add := prevMin1
			if k == prevIdx {
				add = prevMin2
			}
			c := base + add
			if c < curMin1 {
				curMin2, curMin1, curIdx = curMin1, c, k
			} else if c < curMin2 {
				curMin2 = c
			}
		}
		prevMin1, prevMin2, prevIdx = curMin1, curMin2, curIdx
	}
	return prevMin1
}

func main() {
	costs := [][]int{{1, 5, 3}, {2, 9, 4}}
	fmt.Println(minCost(costs)) // expected 5
}
