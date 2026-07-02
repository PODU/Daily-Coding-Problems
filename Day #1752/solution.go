// Day 1752: Remove kth-from-last node of a singly linked list in ONE pass, O(1) space.
// Two pointers spaced k apart; when fast reaches end, slow is just before the target. O(n) time.
// Assumption (no README example): list 1->2->3->4->5, k=2 removes value 4 -> "1 2 3 5".
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func removeKthLast(head *Node, k int) *Node {
	dummy := &Node{next: head}
	fast, slow := dummy, dummy
	for i := 0; i < k; i++ { // advance fast k steps
		fast = fast.next
	}
	for fast.next != nil {
		fast = fast.next
		slow = slow.next
	}
	slow.next = slow.next.next // unlink target
	return dummy.next
}

func build(values []int) *Node {
	dummy := &Node{}
	cur := dummy
	for _, v := range values {
		cur.next = &Node{val: v}
		cur = cur.next
	}
	return dummy.next
}

func main() {
	head := build([]int{1, 2, 3, 4, 5})
	head = removeKthLast(head, 2)
	var parts []string
	for p := head; p != nil; p = p.next {
		parts = append(parts, fmt.Sprintf("%d", p.val))
	}
	fmt.Println(strings.Join(parts, " ")) // 1 2 3 5
}
