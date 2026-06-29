// Binary search for fixed point (A[i]==i) in sorted distinct array; A[i]-i non-decreasing.
// Time: O(log n), Space: O(1).
package main

import (
	"fmt"
	"strconv"
)

func fixedPoint(a []int) string {
	lo, hi, ans := 0, len(a)-1, -1
	for lo <= hi {
		mid := (lo + hi) / 2
		if a[mid] == mid {
			ans = mid
			hi = mid - 1
		} else if a[mid] < mid {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	if ans == -1 {
		return "False"
	}
	return strconv.Itoa(ans)
}

func main() {
	fmt.Println(fixedPoint([]int{-6, 0, 2, 40}))
	fmt.Println(fixedPoint([]int{1, 5, 7, 8}))
}
