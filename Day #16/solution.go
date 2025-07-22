// Approach: Circular (ring) buffer of size N. record/get_last are O(1); O(N) space.
package main

import "fmt"

type OrderLog struct {
	buf   []int
	n     int
	count int
	head  int // next write position
}

func NewOrderLog(n int) *OrderLog {
	return &OrderLog{buf: make([]int, n), n: n}
}

func (l *OrderLog) Record(orderID int) {
	l.buf[l.head] = orderID
	l.head = (l.head + 1) % l.n
	if l.count < l.n {
		l.count++
	}
}

// i is 1-based: GetLast(1) is the most recent
func (l *OrderLog) GetLast(i int) int {
	idx := ((l.head-i)%l.n + l.n) % l.n
	return l.buf[idx]
}

func main() {
	log := NewOrderLog(3)
	for _, x := range []int{1, 2, 3, 4, 5} {
		log.Record(x)
	}
	fmt.Printf("get_last(1) = %d\n", log.GetLast(1))
	fmt.Printf("get_last(2) = %d\n", log.GetLast(2))
	fmt.Printf("get_last(3) = %d\n", log.GetLast(3))
}
