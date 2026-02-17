// Count smaller elements to the right via Fenwick tree + coordinate compression.
// Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func countSmaller(a []int) []int {
	uniq := append([]int{}, a...)
	sort.Ints(uniq)
	uniq = uniq[:0]
	seen := map[int]bool{}
	tmp := append([]int{}, a...)
	sort.Ints(tmp)
	for _, v := range tmp {
		if !seen[v] {
			seen[v] = true
			uniq = append(uniq, v)
		}
	}
	rank := map[int]int{}
	for i, v := range uniq {
		rank[v] = i + 1
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
	res := make([]int, len(a))
	for i := len(a) - 1; i >= 0; i-- {
		rk := rank[a[i]]
		res[i] = query(rk - 1)
		update(rk)
	}
	return res
}

func main() {
	fmt.Println(countSmaller([]int{3, 4, 9, 6, 1}))
}
