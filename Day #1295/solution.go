// Day 1295: Fixed-size log of last N order ids via circular buffer.
// record and getLast are both O(1) time, O(N) space.
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

func (l *OrderLog) GetLast(i int) int64 { // 1 = most recent
	idx := ((l.head-i)%l.n + l.n) % l.n
	return l.buf[idx]
}

func main() {
	log := NewOrderLog(3)
	log.Record(10)
	log.Record(20)
	log.Record(30)
	fmt.Println(log.GetLast(1)) // 30
	fmt.Println(log.GetLast(2)) // 20
	log.Record(40)
	fmt.Println(log.GetLast(1)) // 40
	fmt.Println(log.GetLast(3)) // 20
}
