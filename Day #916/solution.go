// Reverse a singly linked list in-place by re-pointing each node's next pointer.
// Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func reverse(head *Node) *Node {
	var prev *Node
	for head != nil {
		nx := head.next
		head.next = prev
		prev = head
		head = nx
	}
	return prev
}

func toStr(h *Node) string {
	parts := []string{}
	for h != nil {
		parts = append(parts, fmt.Sprintf("%d", h.val))
		h = h.next
	}
	return strings.Join(parts, " -> ")
}

func main() {
	head := &Node{1, &Node{2, &Node{3, &Node{4, &Node{5, nil}}}}}
	head = reverse(head)
	fmt.Println(toStr(head)) // 5 -> 4 -> 3 -> 2 -> 1
}
