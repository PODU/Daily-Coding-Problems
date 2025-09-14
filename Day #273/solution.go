// Day 273: Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log N), Space O(1). Returns index or -1 (False).
package main

import "fmt"

func fixedPoint(a []int) int {
	lo, hi := 0, len(a)-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
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

func show(a []int) {
	if r := fixedPoint(a); r == -1 {
		fmt.Println("False")
	} else {
		fmt.Println(r)
	}
}

func main() {
	show([]int{-6, 0, 2, 40}) // 2
	show([]int{1, 5, 7, 8})   // False
}
