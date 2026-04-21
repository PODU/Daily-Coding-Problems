// DP over rows, tracking the two smallest running totals so each house picks
// the best previous color != its own. Time O(N*K), Space O(1) extra.
package main

import (
	"fmt"
	"math"
)

func minCost(costs [][]int) int {
	if len(costs) == 0 {
		return 0
	}
	K := len(costs[0])
	prev := append([]int(nil), costs[0]...)
	for i := 1; i < len(costs); i++ {
		m1, m2, idx1 := math.MaxInt32, math.MaxInt32, -1
		for k := 0; k < K; k++ {
			if prev[k] < m1 {
				m2, m1, idx1 = m1, prev[k], k
			} else if prev[k] < m2 {
				m2 = prev[k]
			}
		}
		cur := make([]int, K)
		for k := 0; k < K; k++ {
			if k == idx1 {
				cur[k] = costs[i][k] + m2
			} else {
				cur[k] = costs[i][k] + m1
			}
		}
		prev = cur
	}
	ans := math.MaxInt32
	for _, v := range prev {
		if v < ans {
			ans = v
		}
	}
	return ans
}

func main() {
	fmt.Println(minCost([][]int{{1, 5, 3}, {2, 9, 4}})) // 5
}
