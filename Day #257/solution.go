// Day 257: Smallest window that must be sorted to make the whole array sorted.
// Left->right track max to find right bound; right->left track min to find left bound.
// Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"math"
)

func sortWindow(a []int) (int, int) {
	n := len(a)
	begin, end := -1, -1
	mx := math.MinInt64
	for i := 0; i < n; i++ {
		if a[i] < mx {
			end = i
		} else {
			mx = a[i]
		}
	}
	mn := math.MaxInt64
	for i := n - 1; i >= 0; i-- {
		if a[i] > mn {
			begin = i
		} else {
			mn = a[i]
		}
	}
	return begin, end
}

func main() {
	a := []int{3, 7, 5, 6, 9}
	b, e := sortWindow(a)
	fmt.Printf("(%d, %d)\n", b, e) // (1, 3)
}
