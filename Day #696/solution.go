// Day 696: 24-hour subscriber array with point update + inclusive range-sum query.
// Approach: Fenwick (Binary Indexed) Tree. update O(log n), query O(log n), space O(n).
package main

import "fmt"

type Fenwick struct {
	t []int64
	n int
}

func newFenwick(n int) *Fenwick { return &Fenwick{make([]int64, n+1), n} }

func (f *Fenwick) add(i int, v int64) {
	for i++; i <= f.n; i += i & -i {
		f.t[i] += v
	}
}

func (f *Fenwick) pref(i int) int64 {
	var s int64
	for i++; i > 0; i -= i & -i {
		s += f.t[i]
	}
	return s
}

func (f *Fenwick) rng(l, r int) int64 {
	if l > 0 {
		return f.pref(r) - f.pref(l-1)
	}
	return f.pref(r)
}

type Subscribers struct{ f *Fenwick }

func (s *Subscribers) update(hour int, value int64) { s.f.add(hour, value) }
func (s *Subscribers) query(start, end int) int64   { return s.f.rng(start, end) }

func main() {
	s := &Subscribers{newFenwick(24)}
	s.update(3, 10)
	s.update(5, 7)
	s.update(10, 4)
	fmt.Println(s.query(3, 10)) // 21
	fmt.Println(s.query(0, 4))  // 10
	s.update(3, 5)
	fmt.Println(s.query(3, 10)) // 26
}
