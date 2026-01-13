// Paint houses: DP tracking two smallest costs of previous row -> O(N*K) time, O(1) extra space.
// For each house we only need the min and second-min of the previous row to avoid same color.
package main

import (
	"fmt"
	"math"
)

func minCost(costs [][]int) int {
	if len(costs) == 0 {
		return 0
	}
	prevMin, prevSecond, prevIdx := 0, 0, -1
	for _, row := range costs {
		curMin, curSecond, curIdx := math.MaxInt32, math.MaxInt32, -1
		for c, base := range row {
			cost := base + prevMin
			if c == prevIdx {
				cost = base + prevSecond
			}
			if cost < curMin {
				curSecond = curMin
				curMin = cost
				curIdx = c
			} else if cost < curSecond {
				curSecond = cost
			}
		}
		prevMin, prevSecond, prevIdx = curMin, curSecond, curIdx
	}
	return prevMin
}

func main() {
	costs := [][]int{{1, 5, 3}, {2, 9, 4}}
	fmt.Println(minCost(costs))
}
