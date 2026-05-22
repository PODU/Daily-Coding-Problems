// Day 1546: Stable partition of a linked list around pivot k.
// Build two sublists (< k) and (>= k) preserving order, then splice. Time O(n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func partition(head *Node, k int) *Node {
	lessD := &Node{}
	geD := &Node{}
	l, g := lessD, geD
	for c := head; c != nil; c = c.next {
		if c.val < k {
			l.next = c
			l = c
		} else {
			g.next = c
			g = c
		}
	}
	g.next = nil
	l.next = geD.next
	return lessD.next
}

func build(vals []int) *Node {
	var head, tail *Node
	for _, v := range vals {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	return head
}

func toStr(head *Node) string {
	var parts []string
	for c := head; c != nil; c = c.next {
		parts = append(parts, fmt.Sprintf("%d", c.val))
	}
	return strings.Join(parts, " -> ")
}

func main() {
	head := build([]int{5, 1, 8, 0, 3})
	head = partition(head, 3)
	fmt.Println(toStr(head))
}
