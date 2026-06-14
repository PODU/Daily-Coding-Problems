// Smallest unsorted window. Scan L->R tracking max for end, R->L tracking min for start. O(n) time, O(1) space.
package main

import (
	"fmt"
	"math"
)

func unsortedWindow(a []int) (int, int) {
	n := len(a)
	end, start := -1, -1
	mx, mn := math.MinInt, math.MaxInt
	for i := 0; i < n; i++ {
		if a[i] > mx {
			mx = a[i]
		}
		if a[i] < mx {
			end = i
		}
	}
	for i := n - 1; i >= 0; i-- {
		if a[i] < mn {
			mn = a[i]
		}
		if a[i] > mn {
			start = i
		}
	}
	return start, end
}

func main() {
	s, e := unsortedWindow([]int{3, 7, 5, 6, 9})
	fmt.Printf("(%d, %d)\n", s, e)
}
