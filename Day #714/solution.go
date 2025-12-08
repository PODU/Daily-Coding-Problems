// Day 714: Swap every two adjacent nodes in a singly linked list. Iterative
// pointer rewiring with a dummy head. Time O(n), space O(1).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func swapPairs(head *Node) *Node {
	dummy := &Node{next: head}
	prev := dummy
	for prev.next != nil && prev.next.next != nil {
		a := prev.next
		b := a.next
		a.next = b.next
		b.next = a
		prev.next = b
		prev = a
	}
	return dummy.next
}

func main() {
	head := &Node{1, &Node{2, &Node{3, &Node{4, nil}}}}
	head = swapPairs(head)
	parts := []string{}
	for c := head; c != nil; c = c.next {
		parts = append(parts, fmt.Sprintf("%d", c.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
