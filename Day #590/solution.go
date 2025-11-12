// XOR doubly linked list using real address arithmetic via unsafe.Pointer/uintptr.
// both = (uintptr)prev XOR (uintptr)next; traverse with XOR.
// add: O(1), get(i): O(i). Space O(n).
package main

import (
	"fmt"
	"unsafe"
)

type Node struct {
	value int
	both  uintptr // XOR of prev and next addresses
}

type XorList struct {
	head *Node
	tail *Node
}

func ptr(n *Node) uintptr {
	return uintptr(unsafe.Pointer(n))
}

func (l *XorList) add(element int) {
	node := &Node{value: element}
	// keep node alive (escapes to heap because pointer stored), GC-safe via slice
	keep = append(keep, node)
	if l.head == nil {
		l.head = node
		l.tail = node
		return
	}
	node.both = ptr(l.tail) ^ 0
	l.tail.both = (l.tail.both ^ 0) ^ ptr(node)
	l.tail = node
}

func (l *XorList) get(index int) int {
	var prev uintptr = 0
	cur := l.head
	for i := 0; i < index; i++ {
		next := prev ^ cur.both
		prev = ptr(cur)
		cur = (*Node)(unsafe.Pointer(next))
	}
	return cur.value
}

var keep []*Node

func main() {
	list := &XorList{}
	list.add(10)
	list.add(20)
	list.add(30)
	list.add(40)
	fmt.Println(list.get(0))
	fmt.Println(list.get(3))
}
