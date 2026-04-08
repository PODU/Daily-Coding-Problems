// Day 1324: Point-update / range-sum over a 24-element array using a Fenwick (Binary Indexed) Tree.
// update O(log n), query O(log n). 1-indexed internally over fixed size 24.
package main

import "fmt"

type Subscribers struct{ tree [25]int }

func (s *Subscribers) update(hour, value int) {
	for i := hour + 1; i <= 24; i += i & (-i) {
		s.tree[i] += value
	}
}

func (s *Subscribers) prefix(hour int) int {
	sum := 0
	for i := hour + 1; i > 0; i -= i & (-i) {
		sum += s.tree[i]
	}
	return sum
}

func (s *Subscribers) query(start, end int) int {
	left := 0
	if start > 0 {
		left = s.prefix(start - 1)
	}
	return s.prefix(end) - left
}

func main() {
	s := &Subscribers{}
	s.update(2, 10)
	s.update(5, 3)
	s.update(5, 7)
	fmt.Println(s.query(2, 5))  // 20
	fmt.Println(s.query(0, 23)) // 20
	fmt.Println(s.query(3, 4))  // 0
}
