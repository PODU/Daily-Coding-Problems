// Nearest larger: expand outward from i, return first index with greater value.
// Time: O(n), Space: O(1).
package main

import "fmt"

func nearestLarger(a []int, i int) (int, bool) {
	n := len(a)
	for d := 1; d < n; d++ {
		if i-d >= 0 && a[i-d] > a[i] {
			return i - d, true
		}
		if i+d < n && a[i+d] > a[i] {
			return i + d, true
		}
	}
	return 0, false
}

func main() {
	a := []int{4, 1, 3, 5, 6}
	r, ok := nearestLarger(a, 0)
	if !ok {
		fmt.Println("null")
	} else {
		fmt.Println(r)
	}
}
