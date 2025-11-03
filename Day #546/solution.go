// Count smaller elements to the right via Fenwick/BIT + coordinate compression.
// Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type BIT struct {
	t []int
}

func newBIT(n int) *BIT { return &BIT{t: make([]int, n+1)} }

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
	n := len(a)
	tmp := append([]int(nil), a...)
	sort.Ints(tmp)
	uniq := tmp[:0]
	for i, v := range tmp {
		if i == 0 || v != tmp[i-1] {
			uniq = append(uniq, v)
		}
	}
	bit := newBIT(len(uniq))
	res := make([]int, n)
	for i := n - 1; i >= 0; i-- {
		rank := sort.SearchInts(uniq, a[i]) + 1
		res[i] = bit.query(rank - 1)
		bit.update(rank, 1)
	}
	return res
}

func main() {
	a := []int{3, 4, 9, 6, 1}
	res := countSmaller(a)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
