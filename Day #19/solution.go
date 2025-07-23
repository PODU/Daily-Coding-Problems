// Paint House: DP tracking min cost per color using min1/min2 trick.
// Time O(N*K), Space O(1) extra.
package main

import "fmt"

func minCost(cost [][]int) int {
	if len(cost) == 0 {
		return 0
	}
	K := len(cost[0])
	prev := make([]int, K)
	copy(prev, cost[0])
	for i := 1; i < len(cost); i++ {
		min1, min2 := -1, -1
		for k := 0; k < K; k++ {
			if min1 == -1 || prev[k] < prev[min1] {
				min2, min1 = min1, k
			} else if min2 == -1 || prev[k] < prev[min2] {
				min2 = k
			}
		}
		cur := make([]int, K)
		for k := 0; k < K; k++ {
			best := prev[min1]
			if k == min1 {
				best = prev[min2]
			}
			cur[k] = cost[i][k] + best
		}
		prev = cur
	}
	ans := prev[0]
	for _, v := range prev {
		if v < ans {
			ans = v
		}
	}
	return ans
}

func main() {
	cost := [][]int{{1, 5, 3}, {2, 9, 4}}
	fmt.Printf("Minimum cost = %d\n", minCost(cost))
}
