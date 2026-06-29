// Iterative in-place reversal of a singly linked list using three pointers.
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

func main() {
	var head *Node
	for i := 5; i >= 1; i-- {
		head = &Node{val: i, next: head}
	}
	head = reverse(head)
	var out []string
	for c := head; c != nil; c = c.next {
		out = append(out, strconv.Itoa(c.val))
	}
	fmt.Println(strings.Join(out, " "))
}
