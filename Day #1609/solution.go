// Subscribers-per-hour over 24 hours via Fenwick/BIT. update(hour,val)+=, query(start,end)=inclusive range sum.
// Time O(log n) per op, Space O(n).
package main

import "fmt"

type Fenwick struct {
	n    int
	tree []int64
}

func NewFenwick(n int) *Fenwick { return &Fenwick{n: n, tree: make([]int64, n+1)} }

func (f *Fenwick) Update(i int, v int64) {
	for i++; i <= f.n; i += i & -i {
		f.tree[i] += v
	}
}

func (f *Fenwick) pref(i int) int64 {
	var s int64
	for i++; i > 0; i -= i & -i {
		s += f.tree[i]
	}
	return s
}

func (f *Fenwick) Query(l, r int) int64 {
	if l > 0 {
		return f.pref(r) - f.pref(l-1)
	}
	return f.pref(r)
}

func main() {
	bit := NewFenwick(24) // all zeros
	bit.Update(0, 5)
	bit.Update(3, 10)
	bit.Update(23, 2)
	fmt.Println(bit.Query(0, 23)) // 17
	fmt.Println(bit.Query(0, 3))  // 15
	fmt.Println(bit.Query(1, 2))  // 0
}
