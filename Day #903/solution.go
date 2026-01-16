// Sort, then fix arr[i] and two-pointer scan for the remaining pair summing to k.
// Time O(n^2), Space O(1) extra.
package main

import (
	"fmt"
	"sort"
)

func threeSum(arr []int, k int) bool {
	sort.Ints(arr)
	n := len(arr)
	for i := 0; i < n-2; i++ {
		lo, hi := i+1, n-1
		for lo < hi {
			s := arr[i] + arr[lo] + arr[hi]
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
	arr := []int{20, 303, 3, 4, 25}
	if threeSum(arr, 49) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
