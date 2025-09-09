// Fenwick/Binary Indexed Tree over 24 hours.
// update: O(log n), query (prefix-diff): O(log n). Space O(n).
package main

import "fmt"

type BIT struct {
	n    int
	tree []int64
}

func NewBIT(n int) *BIT { return &BIT{n: n, tree: make([]int64, n+1)} }

func (b *BIT) add(i int, v int64) { // 0-based index
	for i++; i <= b.n; i += i & (-i) {
		b.tree[i] += v
	}
}

func (b *BIT) prefix(i int) int64 { // sum of [0..i], 0-based
	var s int64
	for i++; i > 0; i -= i & (-i) {
		s += b.tree[i]
	}
	return s
}

func (b *BIT) query(l, r int) int64 { // inclusive [l..r]
	left := int64(0)
	if l > 0 {
		left = b.prefix(l - 1)
	}
	return b.prefix(r) - left
}

func (b *BIT) update(hour int, value int64) { b.add(hour, value) }

func main() {
	bit := NewBIT(24)
	bit.update(2, 5)
	bit.update(5, 3)
	bit.update(23, 10)
	fmt.Printf("query(2,5) = %d\n", bit.query(2, 5))
	fmt.Printf("query(0,23) = %d\n", bit.query(0, 23))
}
