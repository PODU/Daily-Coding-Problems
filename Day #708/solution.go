// Day 708: Fixed point (a[i]==i) in a sorted distinct array via binary search.
// Since values are distinct integers, a[i]-i is monotonic. Time O(log n).
package main

import "fmt"

func fixedPoint(a []int) int {
	lo, hi := 0, len(a)-1
	for lo <= hi {
		mid := (lo + hi) / 2
		if a[mid] == mid {
			return mid
		} else if a[mid] < mid {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return -1
}

func report(a []int) {
	r := fixedPoint(a)
	if r >= 0 {
		fmt.Println(r)
	} else {
		fmt.Println("False")
	}
}

func main() {
	report([]int{-6, 0, 2, 40})
	report([]int{1, 5, 7, 8})
}
