// Partition linked list: stable split into <k and >=k lists, then concatenate.
// Time O(n), Space O(1).
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

func partition(head *Node, k int) *Node {
	lessDummy := &Node{}
	geDummy := &Node{}
	lt := lessDummy
	ge := geDummy
	for cur := head; cur != nil; cur = cur.next {
		if cur.val < k {
			lt.next = cur
			lt = cur
		} else {
			ge.next = cur
			ge = cur
		}
	}
	ge.next = nil
	lt.next = geDummy.next
	return lessDummy.next
}

func main() {
	vals := []int{5, 1, 8, 0, 3}
	var head, tail *Node
	for _, v := range vals {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	head = partition(head, 3)
	var parts []string
	for cur := head; cur != nil; cur = cur.next {
		parts = append(parts, strconv.Itoa(cur.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
