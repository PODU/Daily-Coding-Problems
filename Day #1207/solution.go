// Day 1207: Remove kth-from-last node in one pass, constant space.
// Two pointers k apart; when lead hits end, trail is just before target. Time O(n), Space O(1).
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

func removeKthLast(head *Node, k int) *Node {
	dummy := &Node{next: head}
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
	head = removeKthLast(head, 2) // remove 4
	var out []string
	for p := head; p != nil; p = p.next {
		out = append(out, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(out, " -> ")) // 1 -> 2 -> 3 -> 5
}
