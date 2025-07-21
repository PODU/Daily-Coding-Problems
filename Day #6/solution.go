// XOR linked list simulated with a "memory" slice; addresses are indices (0 = null).
// each node stores both = prevAddr XOR nextAddr. add: O(1), get(i): O(i). Space: O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	both int // prevAddr XOR nextAddr
}

type XorList struct {
	mem  []*Node
	head int
	tail int
}

func NewXorList() *XorList {
	return &XorList{mem: []*Node{nil}} // index 0 reserved as null
}

func (l *XorList) alloc(n *Node) int {
	l.mem = append(l.mem, n)
	return len(l.mem) - 1
}

func (l *XorList) add(val int) {
	addr := l.alloc(&Node{val: val})
	if l.head == 0 {
		l.head, l.tail = addr, addr
		return
	}
	l.mem[l.tail].both ^= addr
	l.mem[addr].both = l.tail
	l.tail = addr
}

func (l *XorList) get(index int) *Node {
	prev, cur := 0, l.head
	for i := 0; i < index && cur != 0; i++ {
		next := l.mem[cur].both ^ prev
		prev = cur
		cur = next
	}
	if cur == 0 {
		return nil
	}
	return l.mem[cur]
}

func main() {
	l := NewXorList()
	for _, v := range []int{10, 20, 30, 40} {
		l.add(v)
	}
	parts := []string{}
	for i := 0; i < 4; i++ {
		parts = append(parts, fmt.Sprintf("%d", l.get(i).val))
	}
	fmt.Println(strings.Join(parts, " "))
}
