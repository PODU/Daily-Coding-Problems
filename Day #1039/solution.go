// Fenwick/BIT over 24 hours: point update, prefix-sum range query.
// update O(log n), query O(log n).
package main

import "fmt"

type BIT struct {
	n    int
	tree []int64
}

func NewBIT(n int) *BIT {
	return &BIT{n: n, tree: make([]int64, n+1)}
}

func (b *BIT) update(hour int, value int64) {
	for i := hour + 1; i <= b.n; i += i & (-i) {
		b.tree[i] += value
	}
}

func (b *BIT) prefix(idx int) int64 { // sum of [0..idx]
	var s int64 = 0
	for i := idx + 1; i > 0; i -= i & (-i) {
		s += b.tree[i]
	}
	return s
}

func (b *BIT) query(start, end int) int64 { // inclusive
	if start > 0 {
		return b.prefix(end) - b.prefix(start-1)
	}
	return b.prefix(end)
}

func main() {
	bit := NewBIT(24)
	bit.update(0, 5)
	bit.update(3, 10)
	bit.update(23, 2)
	bit.update(3, 1)
	fmt.Printf("query(0, 3) = %d\n", bit.query(0, 3))
	fmt.Printf("query(0, 23) = %d\n", bit.query(0, 23))
	fmt.Printf("query(4, 23) = %d\n", bit.query(4, 23))
	fmt.Printf("query(3, 3) = %d\n", bit.query(3, 3))
}
