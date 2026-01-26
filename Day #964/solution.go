// Day 964: Order log keeping last N ids with O(1) record/get_last.
// Approach: fixed-size circular buffer. Time O(1) per op, Space O(N).
package main

import "fmt"

type OrderLog struct {
	buf   []int
	n     int
	count int
}

func NewOrderLog(n int) *OrderLog { return &OrderLog{buf: make([]int, n), n: n} }

func (o *OrderLog) Record(orderID int) { o.buf[o.count%o.n] = orderID; o.count++ }

func (o *OrderLog) GetLast(i int) int { return o.buf[((o.count-i)%o.n+o.n)%o.n] }

func main() {
	log := NewOrderLog(3)
	for _, x := range []int{10, 20, 30, 40} { // 40 evicts 10
		log.Record(x)
	}
	fmt.Println(log.GetLast(1)) // 40
	fmt.Println(log.GetLast(2)) // 30
	fmt.Println(log.GetLast(3)) // 20
}
