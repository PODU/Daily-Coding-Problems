// Reverse a singly linked list in-place by re-pointing each next pointer.
// Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func reverse(head *Node) *Node {
	var prev *Node
	for head != nil {
		nxt := head.next
		head.next = prev
		prev = head
		head = nxt
	}
	return prev
}

func toStr(head *Node) string {
	var parts []string
	for head != nil {
		parts = append(parts, strconv.Itoa(head.val))
		head = head.next
	}
	return strings.Join(parts, " ")
}

func main() {
	head := &Node{1, &Node{2, &Node{3, &Node{4, &Node{5, nil}}}}}
	fmt.Println(toStr(head))
	head = reverse(head)
	fmt.Println(toStr(head))
}
