// Rotate list right by k: find length L, make a ring, break at L-(k%L).
// Time O(n), Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	Val  int
	Next *Node
}

func build(vals []int) *Node {
	dummy := &Node{}
	cur := dummy
	for _, v := range vals {
		cur.Next = &Node{Val: v}
		cur = cur.Next
	}
	return dummy.Next
}

func toStr(head *Node) string {
	var parts []string
	for head != nil {
		parts = append(parts, strconv.Itoa(head.Val))
		head = head.Next
	}
	return strings.Join(parts, " -> ")
}

func rotateRight(head *Node, k int) *Node {
	if head == nil || head.Next == nil {
		return head
	}
	L := 1
	tail := head
	for tail.Next != nil {
		tail = tail.Next
		L++
	}
	k %= L
	if k == 0 {
		return head
	}
	tail.Next = head // ring
	steps := L - k
	newTail := head
	for i := 0; i < steps-1; i++ {
		newTail = newTail.Next
	}
	newHead := newTail.Next
	newTail.Next = nil
	return newHead
}

func main() {
	head := build([]int{1, 2, 3, 4, 5})
	fmt.Println(toStr(rotateRight(head, 3)))
}
