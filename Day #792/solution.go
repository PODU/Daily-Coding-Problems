// Pyramid: left[i]=min(h,left[i-1]+1), right[i]=min(h,right[i+1]+1), cap=min(left,right).
// Peak P=max(cap); target descends from P both sides. cost=sum(h)-sum(target). O(n) time/space.
package main

import (
	"fmt"
	"strings"
)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	h := []int{1, 1, 3, 3, 2, 1}
	n := len(h)
	left := make([]int, n)
	right := make([]int, n)
	left[0] = min(h[0], 1)
	for i := 1; i < n; i++ {
		left[i] = min(h[i], left[i-1]+1)
	}
	right[n-1] = min(h[n-1], 1)
	for i := n - 2; i >= 0; i-- {
		right[i] = min(h[i], right[i+1]+1)
	}
	P, k := 0, 0
	cap := make([]int, n)
	for i := 0; i < n; i++ {
		cap[i] = min(left[i], right[i])
		if cap[i] > P {
			P = cap[i]
			k = i
		}
	}
	target := make([]int, n)
	target[k] = P
	for j := 1; k-j >= 0; j++ {
		target[k-j] = max(0, P-j)
	}
	for j := 1; k+j < n; j++ {
		target[k+j] = max(0, P-j)
	}
	cost := 0
	parts := make([]string, n)
	for i := 0; i < n; i++ {
		cost += h[i] - target[i]
		parts[i] = fmt.Sprintf("%d", target[i])
	}
	fmt.Printf("%d (resulting in [%s])\n", cost, strings.Join(parts, ", "))
}
