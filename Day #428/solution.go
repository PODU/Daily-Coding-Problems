// Day 428: Min cost to carve a strict pyramid (1,2,..,P,..,2,1); only lowering allowed.
// left[i]/right[i] cap each stone by distance from the ends; peak H = max min(left,right).
// A strict tent of peak H has sum H*H, so cost = sum(stones) - H*H. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	stones := []int{1, 1, 3, 3, 2, 1}
	n := len(stones)
	left := make([]int, n)
	right := make([]int, n)
	left[0] = min(stones[0], 1)
	for i := 1; i < n; i++ {
		left[i] = min(stones[i], left[i-1]+1)
	}
	right[n-1] = min(stones[n-1], 1)
	for i := n - 2; i >= 0; i-- {
		right[i] = min(stones[i], right[i+1]+1)
	}
	H, p := 0, 0
	for i := 0; i < n; i++ {
		b := min(left[i], right[i])
		if b > H {
			H = b
			p = i
		}
	}
	total := 0
	for _, x := range stones {
		total += x
	}
	cost := total - H*H
	parts := make([]string, n)
	for i := 0; i < n; i++ {
		h := H - abs(i-p)
		if h < 0 {
			h = 0
		}
		parts[i] = strconv.Itoa(h)
	}
	fmt.Printf("%d  (resulting in [%s])\n", cost, strings.Join(parts, ", "))
}
