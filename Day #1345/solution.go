// Find smallest window to sort: right bound = last i where a[i] < running max; left bound = first j where a[j] > running min from right.
// Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"math"
)

func findUnsorted(a []int) (int, int) {
	n := len(a)
	end, mx := -1, math.MinInt64
	for i := 0; i < n; i++ {
		if a[i] < mx {
			end = i
		} else {
			mx = a[i]
		}
	}
	start, mn := -1, math.MaxInt64
	for i := n - 1; i >= 0; i-- {
		if a[i] > mn {
			start = i
		} else {
			mn = a[i]
		}
	}
	return start, end
}

func main() {
	a := []int{3, 7, 5, 6, 9}
	start, end := findUnsorted(a)
	fmt.Printf("(%d, %d)\n", start, end)
}
