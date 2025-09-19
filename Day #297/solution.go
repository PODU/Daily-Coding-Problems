// Exact set cover via BFS/DP over bitmask of customers; each drink = bitmask of customers accepting it.
// Time: O(2^m * drinks), Space: O(2^m)  (m = number of customers, small).
package main

import "fmt"

func main() {
	preferences := map[int][]int{
		0: {0, 1, 3, 6},
		1: {1, 4, 7},
		2: {2, 4, 7, 5},
		3: {3, 2, 5},
		4: {5, 8},
	}

	// customers indexed 0..m-1 by their key (keys happen to be 0..4)
	custIdx := map[int]int{}
	i := 0
	for c := 0; c < len(preferences); c++ {
		if _, ok := preferences[c]; ok {
			custIdx[c] = i
			i++
		}
	}
	m := len(preferences)
	full := (1 << m) - 1

	drinkMask := map[int]int{}
	for c, drinks := range preferences {
		for _, d := range drinks {
			drinkMask[d] |= 1 << custIdx[c]
		}
	}

	const INF = 1 << 30
	dp := make([]int, full+1)
	for j := range dp {
		dp[j] = INF
	}
	dp[0] = 0
	queue := []int{0}
	for len(queue) > 0 {
		mask := queue[0]
		queue = queue[1:]
		if mask == full {
			continue
		}
		for _, dm := range drinkMask {
			nm := mask | dm
			if dp[nm] > dp[mask]+1 {
				dp[nm] = dp[mask] + 1
				queue = append(queue, nm)
			}
		}
	}
	fmt.Println(dp[full])
}
