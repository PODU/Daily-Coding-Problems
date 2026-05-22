// Pyramid min cost (only lowering). For each index the max pyramid height is
// min of a left pass (rise +1 from edge) and a right pass. A pyramid of peak h
// retains h*h stones, so cost = sum(a) - max(h)^2. Time O(n), Space O(n).
package main

import "fmt"

func min(x, y int) int {
	if x < y {
		return x
	}
	return y
}

func minCost(a []int) int {
	n := len(a)
	left := make([]int, n)
	right := make([]int, n)
	left[0] = min(a[0], 1)
	for i := 1; i < n; i++ {
		left[i] = min(a[i], left[i-1]+1)
	}
	right[n-1] = min(a[n-1], 1)
	for i := n - 2; i >= 0; i-- {
		right[i] = min(a[i], right[i+1]+1)
	}
	bestPeak, sum := 0, 0
	for i := 0; i < n; i++ {
		h := min(left[i], right[i])
		if h > bestPeak {
			bestPeak = h
		}
		sum += a[i]
	}
	return sum - bestPeak*bestPeak
}

func main() {
	fmt.Println(minCost([]int{1, 1, 3, 3, 2, 1}))
}
