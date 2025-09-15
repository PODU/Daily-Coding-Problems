// Day 282: Detect Pythagorean triplet. Square + sort, then two-pointer per c.
// Time O(N^2), Space O(N) for squares.
package main

import (
	"fmt"
	"sort"
)

func hasTriplet(arr []int64) bool {
	a := make([]int64, len(arr))
	for i, x := range arr {
		a[i] = x * x
	}
	sort.Slice(a, func(i, j int) bool { return a[i] < a[j] })
	n := len(a)
	for c := n - 1; c >= 2; c-- {
		lo, hi := 0, c-1
		for lo < hi {
			s := a[lo] + a[hi]
			if s == a[c] {
				return true
			} else if s < a[c] {
				lo++
			} else {
				hi--
			}
		}
	}
	return false
}

func main() {
	fmt.Println(hasTriplet([]int64{3, 1, 4, 6, 5}))    // true (3,4,5)
	fmt.Println(hasTriplet([]int64{10, 4, 6, 12, 5}))  // false
}
