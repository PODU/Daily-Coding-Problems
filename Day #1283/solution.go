// Day 1283: For each element, count smaller elements to its right.
// Fenwick (BIT) over compressed values, scanning right-to-left. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func countSmaller(a []int) []int {
	n := len(a)
	sorted := append([]int(nil), a...)
	sort.Ints(sorted)
	uniq := sorted[:0]
	for i, v := range sorted {
		if i == 0 || v != sorted[i-1] {
			uniq = append(uniq, v)
		}
	}
	tree := make([]int, len(uniq)+1)
	update := func(i int) {
		for ; i < len(tree); i += i & -i {
			tree[i]++
		}
	}
	query := func(i int) int {
		s := 0
		for ; i > 0; i -= i & -i {
			s += tree[i]
		}
		return s
	}
	res := make([]int, n)
	for i := n - 1; i >= 0; i-- {
		rank := sort.SearchInts(uniq, a[i]) + 1
		res[i] = query(rank - 1)
		update(rank)
	}
	return res
}

func main() {
	fmt.Println(countSmaller([]int{3, 4, 9, 6, 1})) // [1 1 2 1 0]
}
