// Day 1182: Split N into k contiguous partitions minimizing the maximum sum.
// Binary search the answer in [max element, total]; greedy feasibility check.
// Time O(N log(sum)), Space O(1).
package main

import "fmt"

func feasible(a []int, k int, cap int) bool {
	cur, parts := 0, 1
	for _, x := range a {
		if cur+x > cap {
			parts++
			cur = x
			if parts > k {
				return false
			}
		} else {
			cur += x
		}
	}
	return true
}

func splitArray(a []int, k int) int {
	lo, hi := 0, 0
	for _, x := range a {
		if x > lo {
			lo = x
		}
		hi += x
	}
	for lo < hi {
		mid := (lo + hi) / 2
		if feasible(a, k, mid) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}

func main() {
	fmt.Println(splitArray([]int{5, 1, 2, 7, 3, 4}, 3)) // 8
}
