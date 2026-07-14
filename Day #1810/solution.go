// 3-sum decision: does any triple sum to k?
// Sort, then for each i two-pointer scan. Time: O(n^2). Space: O(1).
package main

import (
	"fmt"
	"sort"
)

func threeSum(arr []int, k int) bool {
	a := append([]int(nil), arr...)
	sort.Ints(a)
	n := len(a)
	for i := 0; i < n-2; i++ {
		lo, hi := i+1, n-1
		for lo < hi {
			s := a[i] + a[lo] + a[hi]
			if s == k {
				return true
			} else if s < k {
				lo++
			} else {
				hi--
			}
		}
	}
	return false
}

func main() {
	fmt.Println(threeSum([]int{20, 303, 3, 4, 25}, 49)) // true
}
