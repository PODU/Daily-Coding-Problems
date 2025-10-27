// Day 504: Last-N order log via fixed-size circular buffer.
// record O(1), get_last(i) O(1); space O(N).
package main

import "fmt"

type OrderLog struct {
	buf   []int64
	cap   int
	pos   int // next write index
	count int // records seen (capped at cap)
}

func NewOrderLog(n int) *OrderLog {
	return &OrderLog{buf: make([]int64, n), cap: n}
}

func (l *OrderLog) record(orderID int64) {
	l.buf[l.pos] = orderID
	l.pos = (l.pos + 1) % l.cap
	if l.count < l.cap {
		l.count++
	}
}

// i = 1 is the most recent.
func (l *OrderLog) getLast(i int) int64 {
	idx := ((l.pos-i)%l.cap + l.cap) % l.cap
	return l.buf[idx]
}

func main() {
	log := NewOrderLog(5)
	for _, id := range []int64{1, 2, 3, 4, 5, 6, 7} {
		log.record(id)
	}
	fmt.Printf("get_last(1) = %d\n", log.getLast(1))
	fmt.Printf("get_last(2) = %d\n", log.getLast(2))
	fmt.Printf("get_last(3) = %d\n", log.getLast(3))
}
