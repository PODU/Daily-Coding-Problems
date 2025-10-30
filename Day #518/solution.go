// 3-sum decision: sort, fix one element, two-pointer scan the rest. O(n^2) time, O(1) extra.
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
		for lo < hi {
			s := a[i] + a[lo] + a[hi]
			if s == k {
				return true
			}
			if s < k {
				lo++
			} else {
				hi--
			}
		}
	}
	return false
}

func main() {
	fmt.Println(threeSum([]int{20, 303, 3, 4, 25}, 49))
}
