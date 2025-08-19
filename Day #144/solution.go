// Nearest larger element's index by index-distance: expand outward from i. O(n) per query, O(1) space.
package main

import "fmt"

// returns index, or -1 if none (null)
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
	a := []int{4, 1, 3, 5, 6}
	fmt.Println(nearestLarger(a, 0)) // 3
}
