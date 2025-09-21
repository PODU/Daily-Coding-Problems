// Day 311: Find a peak (greater than both neighbors) when ends are the lowest.
// Binary search toward the rising side. O(log N).
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
	return lo
}

func main() {
	a := []int{1, 3, 5, 7, 6, 4, 2}
	p := findPeak(a)
	fmt.Printf("index %d value %d\n", p, a[p]) // index 3 value 7
}
