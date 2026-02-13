// Day 1062: Swap every two adjacent nodes in a singly linked list.
// Approach: iterative pointer manipulation. Time O(n), Space O(1).
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
	dummy := &Node{}
	dummy.next = head
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

func printList(head *Node) {
	var parts []string
	for head != nil {
		parts = append(parts, fmt.Sprintf("%d", head.val))
		head = head.next
	}
	fmt.Println(strings.Join(parts, " -> "))
}

func main() {
	head := &Node{val: 1}
	head.next = &Node{val: 2}
	head.next.next = &Node{val: 3}
	head.next.next.next = &Node{val: 4}
	head = swapPairs(head)
	printList(head) // 2 -> 1 -> 4 -> 3
}
