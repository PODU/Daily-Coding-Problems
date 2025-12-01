// Count inversions via modified merge sort. Time O(N log N), Space O(N).
package main

import "fmt"

func sortCount(a []int) ([]int, int64) {
	if len(a) <= 1 {
		return a, 0
	}
	mid := len(a) / 2
	left, il := sortCount(a[:mid])
	right, ir := sortCount(a[mid:])
	merged := make([]int, 0, len(a))
	i, j := 0, 0
	var inv int64
	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			merged = append(merged, left[i])
			i++
		} else {
			merged = append(merged, right[j])
			j++
			inv += int64(len(left) - i)
		}
	}
	merged = append(merged, left[i:]...)
	merged = append(merged, right[j:]...)
	return merged, il + ir + inv
}

func countInversions(arr []int) int64 {
	cp := append([]int(nil), arr...)
	_, inv := sortCount(cp)
	return inv
}

func main() {
	fmt.Println(countInversions([]int{2, 4, 1, 3, 5}))
	fmt.Println(countInversions([]int{5, 4, 3, 2, 1}))
}
