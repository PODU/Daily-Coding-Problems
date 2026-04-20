// Reverse singly linked list in-place: iterative 3-pointer (prev/cur/next). O(n) time, O(1) space.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func reverseList(head *Node) *Node {
	var prev *Node
	cur := head
	for cur != nil {
		next := cur.next
		cur.next = prev
		prev = cur
		cur = next
	}
	return prev
}

func main() {
	head := &Node{val: 1}
	head.next = &Node{val: 2}
	head.next.next = &Node{val: 3}
	head.next.next.next = &Node{val: 4}
	head.next.next.next.next = &Node{val: 5}

	head = reverseList(head)

	var parts []string
	for p := head; p != nil; p = p.next {
		parts = append(parts, fmt.Sprintf("%d", p.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
