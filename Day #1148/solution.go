// Day 1148: Rotate linked list right by k.
// Find length, close into ring, cut at (len - k%len). O(n) time, O(1) space.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func rotate(head *Node, k int) *Node {
	if head == nil || head.next == nil {
		return head
	}
	length := 1
	tail := head
	for tail.next != nil {
		tail = tail.next
		length++
	}
	k %= length
	if k == 0 {
		return head
	}
	tail.next = head
	steps := length - k
	newTail := head
	for i := 1; i < steps; i++ {
		newTail = newTail.next
	}
	newHead := newTail.next
	newTail.next = nil
	return newHead
}

func build(vals []int) *Node {
	dummy := &Node{}
	t := dummy
	for _, x := range vals {
		t.next = &Node{val: x}
		t = t.next
	}
	return dummy.next
}

func toStr(h *Node) string {
	var out []string
	for ; h != nil; h = h.next {
		out = append(out, fmt.Sprintf("%d", h.val))
	}
	return strings.Join(out, " -> ")
}

func main() {
	fmt.Println(toStr(rotate(build([]int{7, 7, 3, 5}), 2)))    // 3 -> 5 -> 7 -> 7
	fmt.Println(toStr(rotate(build([]int{1, 2, 3, 4, 5}), 3))) // 3 -> 4 -> 5 -> 1 -> 2
}
