// Partition list: build "less than k" and ">= k" sublists, then join. O(n) time, O(1) extra.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func partition(head *Node, k int) *Node {
	lessDummy, geDummy := &Node{}, &Node{}
	less, ge := lessDummy, geDummy
	for cur := head; cur != nil; cur = cur.next {
		if cur.val < k {
			less.next = cur
			less = cur
		} else {
			ge.next = cur
			ge = cur
		}
	}
	ge.next = nil
	less.next = geDummy.next
	return lessDummy.next
}

func build(vals []int) *Node {
	var head *Node
	for i := len(vals) - 1; i >= 0; i-- {
		head = &Node{vals[i], head}
	}
	return head
}

func show(head *Node) string {
	var parts []string
	for c := head; c != nil; c = c.next {
		parts = append(parts, fmt.Sprint(c.val))
	}
	return strings.Join(parts, " -> ")
}

func main() {
	fmt.Println(show(partition(build([]int{5, 1, 8, 0, 3}), 3)))
}
