// Pyramid reshape (lowering only): L[i]/R[i] cap ramp slopes, peak v=max min(L,R), cost=sum-v*v.
// Time O(n), Space O(n).
package main

import "fmt"

func minCost(h []int) int {
	n := len(h)
	L := make([]int, n)
	R := make([]int, n)
	min := func(a, b int) int { if a < b { return a }; return b }
	max := func(a, b int) int { if a > b { return a }; return b }
	L[0] = min(h[0], 1)
	for i := 1; i < n; i++ {
		L[i] = min(h[i], L[i-1]+1)
	}
	R[n-1] = min(h[n-1], 1)
	for i := n - 2; i >= 0; i-- {
		R[i] = min(h[i], R[i+1]+1)
	}
	v, sum := 0, 0
	for i := 0; i < n; i++ {
		v = max(v, min(L[i], R[i]))
		sum += h[i]
	}
	return sum - v*v
}

func main() {
	h := []int{1, 1, 3, 3, 2, 1}
	fmt.Println(minCost(h))
}
