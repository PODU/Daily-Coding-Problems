// Day 773: Count inversions via modified merge sort. O(n log n) time, O(n) space.
package main

import "fmt"

func mergeCount(a []int, l, r int, tmp []int) int64 {
	if r-l <= 1 {
		return 0
	}
	m := (l + r) / 2
	cnt := mergeCount(a, l, m, tmp) + mergeCount(a, m, r, tmp)
	i, j, k := l, m, l
	for i < m && j < r {
		if a[i] <= a[j] {
			tmp[k] = a[i]
			i++
		} else {
			tmp[k] = a[j]
			j++
			cnt += int64(m - i)
		}
		k++
	}
	for i < m {
		tmp[k] = a[i]
		i++
		k++
	}
	for j < r {
		tmp[k] = a[j]
		j++
		k++
	}
	copy(a[l:r], tmp[l:r])
	return cnt
}

func countInversions(arr []int) int64 {
	a := append([]int(nil), arr...)
	return mergeCount(a, 0, len(a), make([]int, len(a)))
}

func main() {
	fmt.Println(countInversions([]int{2, 4, 1, 3, 5})) // 3
	fmt.Println(countInversions([]int{5, 4, 3, 2, 1})) // 10
}
