// Day 165: Count smaller elements to the right. Coordinate-compress, then sweep
// right to left with a Fenwick tree (BIT). Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

type BIT struct{ t []int }

func (b *BIT) update(i, v int) {
	for ; i < len(b.t); i += i & -i {
		b.t[i] += v
	}
}
func (b *BIT) query(i int) int {
	s := 0
	for ; i > 0; i -= i & -i {
		s += b.t[i]
	}
	return s
}

func countSmaller(a []int) []int {
	sortedUnique := append([]int(nil), a...)
	sort.Ints(sortedUnique)
	uniq := sortedUnique[:0]
	for i, v := range sortedUnique {
		if i == 0 || v != sortedUnique[i-1] {
			uniq = append(uniq, v)
		}
	}
	bit := &BIT{t: make([]int, len(uniq)+1)}
	res := make([]int, len(a))
	for i := len(a) - 1; i >= 0; i-- {
		rank := sort.SearchInts(uniq, a[i]) + 1
		res[i] = bit.query(rank - 1)
		bit.update(rank, 1)
	}
	return res
}

func main() {
	fmt.Println(countSmaller([]int{3, 4, 9, 6, 1})) // [1 1 2 1 0]
}
