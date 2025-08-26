// Rotate singly linked list right by k: form ring, break at (len - k%len). Time O(n), Space O(1).
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

func rotateRight(head *Node, k int) *Node {
	if head == nil || head.next == nil || k == 0 {
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
	tail.next = head // make ring
	steps := length - k
	newTail := head
	for i := 0; i < steps-1; i++ {
		newTail = newTail.next
	}
	newHead := newTail.next
	newTail.next = nil
	return newHead
}

func build(vals []int) *Node {
	dummy := &Node{}
	cur := dummy
	for _, v := range vals {
		cur.next = &Node{val: v}
		cur = cur.next
	}
	return dummy.next
}

func toStr(head *Node) string {
	var parts []string
	for head != nil {
		parts = append(parts, strconv.Itoa(head.val))
		head = head.next
	}
	return strings.Join(parts, " -> ")
}

func main() {
	fmt.Println(toStr(rotateRight(build([]int{7, 7, 3, 5}), 2)))
	fmt.Println(toStr(rotateRight(build([]int{1, 2, 3, 4, 5}), 3)))
}
