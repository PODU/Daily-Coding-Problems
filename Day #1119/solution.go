// Day 1119 - Split array into k contiguous partitions minimizing the max sum
// Binary search the answer; greedily count partitions. Time: O(n log(sum)).
package main

import "fmt"

func partitionsNeeded(N []int, limit int) int {
	count, cur := 1, 0
	for _, x := range N {
		if cur+x > limit {
			count++
			cur = x
		} else {
			cur += x
		}
	}
	return count
}

func splitMinMax(N []int, k int) int {
	lo, hi := 0, 0
	for _, x := range N {
		if x > lo {
			lo = x
		}
		hi += x
	}
	for lo < hi {
		mid := (lo + hi) / 2
		if partitionsNeeded(N, mid) <= k {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}

func main() {
	N := []int{5, 1, 2, 7, 3, 4}
	fmt.Println(splitMinMax(N, 3)) // 8
}
