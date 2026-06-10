// Count inversions using modified merge sort (count cross-pairs during merge).
// Time: O(N log N), Space: O(N).
package main

import "fmt"

func mergeCount(a, tmp []int, lo, hi int) int64 {
	if hi-lo <= 1 {
		return 0
	}
	mid := lo + (hi-lo)/2
	inv := mergeCount(a, tmp, lo, mid) + mergeCount(a, tmp, mid, hi)
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
	for t := lo; t < hi; t++ {
		a[t] = tmp[t]
	}
	return inv
}

func countInversions(src []int) int64 {
	a := make([]int, len(src))
	copy(a, src)
	tmp := make([]int, len(a))
	return mergeCount(a, tmp, 0, len(a))
}

func main() {
	fmt.Printf("[2, 4, 1, 3, 5] has %d inversions\n", countInversions([]int{2, 4, 1, 3, 5}))
	fmt.Printf("[5, 4, 3, 2, 1] has %d inversions\n", countInversions([]int{5, 4, 3, 2, 1}))
}
