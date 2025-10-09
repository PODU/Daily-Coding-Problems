// Remove k-th node from end in one pass via two pointers + dummy head. O(n) time, O(1) space.
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

func removeKthFromEnd(head *Node, k int) *Node {
	dummy := &Node{0, head}
	fast, slow := dummy, dummy
	for i := 0; i < k; i++ { // advance fast k ahead
		fast = fast.next
	}
	for fast.next != nil {
		fast = fast.next
		slow = slow.next
	}
	slow.next = slow.next.next
	return dummy.next
}

func printList(head *Node) {
	var parts []string
	for c := head; c != nil; c = c.next {
		parts = append(parts, strconv.Itoa(c.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}

func main() {
	head := &Node{1, &Node{2, &Node{3, &Node{4, &Node{5, nil}}}}}
	head = removeKthFromEnd(head, 2)
	printList(head)
}
