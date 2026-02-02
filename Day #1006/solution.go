// Count intersecting segment pairs: sort by p, count inversions in q via merge sort.
// Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func mergeCount(a, tmp []int, l, r int) int64 {
	if r-l <= 1 {
		return 0
	}
	m := (l + r) / 2
	cnt := mergeCount(a, tmp, l, m) + mergeCount(a, tmp, m, r)
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
	for t := l; t < r; t++ {
		a[t] = tmp[t]
	}
	return cnt
}

func countIntersections(p, q []int) int64 {
	n := len(p)
	idx := make([]int, n)
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(idx, func(x, y int) bool { return p[idx[x]] < p[idx[y]] })
	qs := make([]int, n)
	for i := 0; i < n; i++ {
		qs[i] = q[idx[i]]
	}
	return mergeCount(qs, make([]int, n), 0, n)
}

func main() {
	p := []int{1, 2, 3, 4}
	q := []int{4, 3, 2, 1}
	fmt.Println(countIntersections(p, q)) // 6
}
