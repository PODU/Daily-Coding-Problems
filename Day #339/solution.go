// Three entries summing to k: sort + fix one + two-pointer.
// O(n^2) time, O(1) extra space.
package main

import (
	"fmt"
	"sort"
)

func threeSum(a []int, k int) bool {
	sort.Ints(a)
	n := len(a)
	for i := 0; i < n-2; i++ {
		lo, hi := i+1, n-1
		target := k - a[i]
		for lo < hi {
			s := a[lo] + a[hi]
			if s == target {
				return true
			}
			if s < target {
				lo++
			} else {
				hi--
			}
		}
	}
	return false
}

func main() {
	a := []int{20, 303, 3, 4, 25}
	fmt.Println(threeSum(a, 49)) // true
}
