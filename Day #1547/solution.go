// XOR linked list simulated with slices indexed by integer addresses.
// add appends in O(1); get traverses with nextAddr = curBoth XOR prevAddr in O(index). Space O(n).
package main

import "fmt"

type XorList struct {
	val  []int // address 0 is sentinel/null
	both []int // prevAddr XOR nextAddr
	head int
	tail int
}

func NewXorList() *XorList {
	return &XorList{val: []int{0}, both: []int{0}}
}

func (l *XorList) add(v int) {
	addr := len(l.val)
	l.val = append(l.val, v)
	l.both = append(l.both, 0)
	if l.head == 0 {
		l.head, l.tail = addr, addr
	} else {
		l.both[l.tail] ^= addr
		l.both[addr] ^= l.tail
		l.tail = addr
	}
}

func (l *XorList) get(index int) int {
	prev, cur := 0, l.head
	for i := 0; i < index; i++ {
		next := l.both[cur] ^ prev
		prev, cur = cur, next
	}
	return l.val[cur]
}

func main() {
	list := NewXorList()
	for _, v := range []int{10, 20, 30, 40, 50} {
		list.add(v)
	}
	fmt.Println(list.get(0))
	fmt.Println(list.get(2))
	fmt.Println(list.get(4))
}
