// XOR linked list via memory-pool simulation: nodes stored in a slice, indices act as
// "addresses"; both = prevId XOR nextId (sentinel 0 = null, real nodes start at 1).
// add O(1) with tail tracking, get O(index). O(1) extra per node.
package main

import "fmt"

type node struct {
	value int
	both  int // prevId XOR nextId
}

type XorList struct {
	pool []*node // index 0 reserved as null sentinel
	head int
	tail int
}

func NewXorList() *XorList {
	return &XorList{pool: []*node{nil}}
}

func (l *XorList) Add(element int) {
	n := &node{value: element}
	l.pool = append(l.pool, n)
	id := len(l.pool) - 1
	if l.head == 0 {
		l.head, l.tail = id, id
	} else {
		l.pool[l.tail].both ^= id // old tail next becomes id
		n.both = l.tail           // prev = old tail, next = 0
		l.tail = id
	}
}

func (l *XorList) Get(index int) int {
	prev, cur := 0, l.head
	for i := 0; i < index && cur != 0; i++ {
		next := l.pool[cur].both ^ prev
		prev, cur = cur, next
	}
	if cur == 0 {
		panic("index out of range")
	}
	return l.pool[cur].value
}

func main() {
	list := NewXorList()
	for _, v := range []int{10, 20, 30, 40, 50} {
		list.Add(v)
	}
	fmt.Printf("get(0) = %d\n", list.Get(0))
	fmt.Printf("get(2) = %d\n", list.Get(2))
	fmt.Printf("get(4) = %d\n", list.Get(4))
}
