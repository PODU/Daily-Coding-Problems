// Day 1838: Count smaller elements to the right, via a Fenwick tree over compressed values.
// Time O(N log N), Space O(N).
package main

import (
	"fmt"
	"sort"
)

type BIT struct{ t []int }

func (b *BIT) add(i int) {
	for ; i < len(b.t); i += i & -i {
		b.t[i]++
	}
}
func (b *BIT) sum(i int) int {
	s := 0
	for ; i > 0; i -= i & -i {
		s += b.t[i]
	}
	return s
}

func countSmaller(a []int) []int {
	seen := map[int]bool{}
	var uniq []int
	for _, v := range a {
		if !seen[v] {
			seen[v] = true
			uniq = append(uniq, v)
		}
	}
	sort.Ints(uniq)
	bit := &BIT{t: make([]int, len(uniq)+1)}
	res := make([]int, len(a))
	for i := len(a) - 1; i >= 0; i-- {
		r := sort.SearchInts(uniq, a[i]) + 1 // 1-indexed rank
		res[i] = bit.sum(r - 1)
		bit.add(r)
	}
	return res
}

func main() {
	fmt.Println(countSmaller([]int{3, 4, 9, 6, 1}))
}
