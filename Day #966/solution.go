// Day 966: Deep clone a linked list where each node has a random pointer.
// Approach: hash map original->copy then wire pointers. Time O(n), Space O(n).
package main

import "fmt"

type Node struct {
	val    int
	next   *Node
	random *Node
}

func cloneList(head *Node) *Node {
	if head == nil {
		return nil
	}
	m := make(map[*Node]*Node)
	for p := head; p != nil; p = p.next {
		m[p] = &Node{val: p.val}
	}
	for p := head; p != nil; p = p.next {
		m[p].next = m[p.next]
		m[p].random = m[p.random]
	}
	return m[head]
}

func main() {
	a := &Node{val: 1}
	b := &Node{val: 2}
	c := &Node{val: 3}
	a.next = b
	b.next = c
	a.random = c
	b.random = a
	c.random = b

	for p := cloneList(a); p != nil; p = p.next {
		rv := -1
		if p.random != nil {
			rv = p.random.val
		}
		fmt.Printf("val=%d random=%d\n", p.val, rv)
	}
}
