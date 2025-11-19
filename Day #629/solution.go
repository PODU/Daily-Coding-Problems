// Split array into k contiguous parts minimizing the max partition sum.
// Binary search on answer in [max, total]; greedy feasibility check. O(n log(sum)).
package main

import "fmt"

func feasible(nums []int, k int, cap int) bool {
	parts, cur := 1, 0
	for _, x := range nums {
		if cur+x > cap {
			parts++
			cur = x
		} else {
			cur += x
		}
	}
	return parts <= k
}

func splitArray(nums []int, k int) int {
	lo, hi := 0, 0
	for _, x := range nums {
		if x > lo {
			lo = x
		}
		hi += x
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		if feasible(nums, k, mid) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}

func main() {
	fmt.Println(splitArray([]int{5, 1, 2, 7, 3, 4}, 3)) // expected 8
}
