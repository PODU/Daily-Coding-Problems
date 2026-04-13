// Remove consecutive nodes summing to zero: dummy head, prefix-sum -> last node map;
// repeated prefix means a zero-sum span to splice out. Time O(n), Space O(n).
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

func removeZeroSum(head *Node) *Node {
	dummy := &Node{0, head}
	seen := map[int]*Node{}
	prefix := 0
	for cur := dummy; cur != nil; cur = cur.next {
		prefix += cur.val
		seen[prefix] = cur // last node achieving this prefix sum
	}
	prefix = 0
	for cur := dummy; cur != nil; cur = cur.next {
		prefix += cur.val
		cur.next = seen[prefix].next // skip zero-sum span
	}
	return dummy.next
}

func build(vals []int) *Node {
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
	return head
}

func main() {
	head := build([]int{3, 4, -7, 5, -6, 6})
	head = removeZeroSum(head)
	var parts []string
	for c := head; c != nil; c = c.next {
		parts = append(parts, strconv.Itoa(c.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
