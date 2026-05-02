// Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log n), Space O(1).
package main

import "fmt"

// fixedPoint returns (index, true) where arr[i]==i, or (-1, false) if none.
func fixedPoint(arr []int) (int, bool) {
	lo, hi := 0, len(arr)-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		if arr[mid] == mid {
			return mid, true
		} else if arr[mid] < mid {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return -1, false
}

func run(arr []int) {
	if idx, ok := fixedPoint(arr); ok {
		fmt.Println(idx)
	} else {
		fmt.Println("False")
	}
}

func main() {
	run([]int{-6, 0, 2, 40})
	run([]int{1, 5, 7, 8})
}
