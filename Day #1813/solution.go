// Last-N order log via fixed-size circular buffer.
// record: O(1), get_last(i): O(1). Space: O(N).
package main

import "fmt"

type OrderLog struct {
	buf   []int64
	n     int
	head  int
	count int
}

func NewOrderLog(n int) *OrderLog {
	return &OrderLog{buf: make([]int64, n), n: n}
}

func (l *OrderLog) Record(id int64) {
	l.buf[l.head] = id
	l.head = (l.head + 1) % l.n
	if l.count < l.n {
		l.count++
	}
}

func (l *OrderLog) GetLast(i int) int64 {
	idx := ((l.head-i)%l.n + l.n) % l.n
	return l.buf[idx]
}

func main() {
	log := NewOrderLog(5)
	for _, id := range []int64{101, 102, 103, 104, 105, 106} {
		log.Record(id)
	}
	fmt.Println(log.GetLast(1)) // 106
	fmt.Println(log.GetLast(3)) // 104
}
