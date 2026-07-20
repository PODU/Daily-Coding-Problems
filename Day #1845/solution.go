// Day 1845: Index of nearest larger value (by array distance) via outward expansion.
// Time O(N) per query, Space O(1). Returns -1 if none exists.
package main

import "fmt"

func nearestLarger(a []int, i int) int {
	n := len(a)
	for d := 1; d < n; d++ {
		if i-d >= 0 && a[i-d] > a[i] {
			return i - d
		}
		if i+d < n && a[i+d] > a[i] {
			return i + d
		}
	}
	return -1
}

func main() {
	fmt.Println(nearestLarger([]int{4, 1, 3, 5, 6}, 0)) // 3
}
