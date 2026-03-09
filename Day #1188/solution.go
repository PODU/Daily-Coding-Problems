// Count inversions via merge sort: count cross-pairs while merging. Time O(N log N), Space O(N).
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
	for x := lo; x < hi; x++ {
		a[x] = tmp[x]
	}
	return inv
}

func main() {
	arr := []int{2, 4, 1, 3, 5}
	tmp := make([]int, len(arr))
	fmt.Println(mergeCount(arr, 0, len(arr), tmp))
}
