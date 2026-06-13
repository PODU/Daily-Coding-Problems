// Rotate singly linked list right by k. Make ring, break at n-(k%n). O(n) time, O(1) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	v    int
	next *Node
}

func rotateRight(head *Node, k int) *Node {
	if head == nil || head.next == nil {
		return head
	}
	n := 1
	tail := head
	for tail.next != nil {
		tail = tail.next
		n++
	}
	tail.next = head // ring
	steps := n - (k % n)
	newTail := head
	for i := 1; i < steps; i++ {
		newTail = newTail.next
	}
	newHead := newTail.next
	newTail.next = nil
	return newHead
}

func build(xs []int) *Node {
	var h, t *Node
	for _, x := range xs {
		nd := &Node{v: x}
		if h == nil {
			h, t = nd, nd
		} else {
			t.next = nd
			t = nd
		}
	}
	return h
}

func show(h *Node) {
	var parts []string
	for p := h; p != nil; p = p.next {
		parts = append(parts, strconv.Itoa(p.v))
	}
	fmt.Println(strings.Join(parts, " -> "))
}

func main() {
	show(rotateRight(build([]int{7, 7, 3, 5}), 2))
	show(rotateRight(build([]int{1, 2, 3, 4, 5}), 3))
}
