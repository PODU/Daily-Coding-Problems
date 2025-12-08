// Day 717: Min cost to paint N houses, K colors, no two adjacent same color.
// DP over rows tracking best & second-best previous costs. Time O(N*K), space O(1).
package main

import "fmt"

func minCost(costs [][]int) int {
	if len(costs) == 0 {
		return 0
	}
	K := len(costs[0])
	prev := append([]int(nil), costs[0]...)
	const INF = int(^uint(0) >> 1)
	for i := 1; i < len(costs); i++ {
		m1, m2, idx := INF, INF, -1
		for k := 0; k < K; k++ {
			if prev[k] < m1 {
				m2, m1, idx = m1, prev[k], k
			} else if prev[k] < m2 {
				m2 = prev[k]
			}
		}
		cur := make([]int, K)
		for k := 0; k < K; k++ {
			add := m1
			if k == idx {
				add = m2
			}
			cur[k] = costs[i][k] + add
		}
		prev = cur
	}
	ans := INF
	for _, v := range prev {
		if v < ans {
			ans = v
		}
	}
	return ans
}

func main() {
	fmt.Println(minCost([][]int{{1, 5, 3}, {2, 9, 4}}))
}
