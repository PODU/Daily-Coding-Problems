// Count Inversions via modified merge sort: count cross-pairs while merging.
// Time O(n log n), Space O(n).
package main

import "fmt"

func mergeCount(a []int, lo, hi int, tmp []int) int64 {
	if hi-lo <= 1 {
		return 0
	}
	mid := (lo + hi) / 2
	inv := mergeCount(a, lo, mid, tmp) + mergeCount(a, mid, hi, tmp)
	i, j, k := lo, mid, lo
	for i < mid && j < hi {
		if a[i] <= a[j] {
			tmp[k] = a[i]
			i++
		} else {
			tmp[k] = a[j]
			j++
			inv += int64(mid - i)
		}
		k++
	}
	for i < mid {
		tmp[k] = a[i]
		i++
		k++
	}
	for j < hi {
		tmp[k] = a[j]
		j++
		k++
	}
	copy(a[lo:hi], tmp[lo:hi])
	return inv
}

func countInversions(src []int) int64 {
	a := make([]int, len(src))
	copy(a, src)
	return mergeCount(a, 0, len(a), make([]int, len(a)))
}

func main() {
	fmt.Println(countInversions([]int{2, 4, 1, 3, 5}))
	fmt.Println(countInversions([]int{5, 4, 3, 2, 1}))
}
