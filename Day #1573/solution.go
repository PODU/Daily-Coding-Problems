// Find a peak in a rotated array (ends lowest) via binary search. O(log N) time, O(1) space.
package main

import "fmt"

func findPeak(a []int) int {
	lo, hi := 0, len(a)-1
	for lo < hi {
		mid := (lo + hi) / 2
		if a[mid] < a[mid+1] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return a[lo]
}

func main() {
	arr := []int{5, 7, 9, 3, 1}
	fmt.Println(findPeak(arr))
}
