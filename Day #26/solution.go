// Remove kth-from-last node in one pass with two pointers. Time O(n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func removeKthFromLast(head *Node, k int) *Node {
	dummy := &Node{0, head}
	lead, trail := dummy, dummy
	for i := 0; i < k; i++ {
		lead = lead.next
	}
	for lead.next != nil {
		lead = lead.next
		trail = trail.next
	}
	trail.next = trail.next.next
	return dummy.next
}

func main() {
	head := &Node{1, &Node{2, &Node{3, &Node{4, &Node{5, nil}}}}}
	head = removeKthFromLast(head, 2)
	var parts []string
	for c := head; c != nil; c = c.next {
		parts = append(parts, fmt.Sprintf("%d", c.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
