// Expand outward from index i, returning nearest j (by |j-i|) with a[j] > a[i]; -1 if none.
// Time: O(n) per query, Space: O(1).
package main

import "fmt"

func nearestLarger(a []int, i int) int {
	n := len(a)
	for d := 1; d < n; d++ {
		l, r := i-d, i+d
		if l >= 0 && a[l] > a[i] {
			return l
		}
		if r < n && a[r] > a[i] {
			return r
		}
	}
	return -1
}

func main() {
	a := []int{4, 1, 3, 5, 6}
	fmt.Println(nearestLarger(a, 0))
}
