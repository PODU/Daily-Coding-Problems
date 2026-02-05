// Day 1025: Remove all consecutive linked-list nodes that sum to zero.
// Approach: prefix-sum + map of last node per prefix sum (dummy head). O(N) time, O(N) space.
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
	last := make(map[int]*Node)
	sum := 0
	for cur := dummy; cur != nil; cur = cur.next {
		sum += cur.val
		last[sum] = cur // keep last occurrence
	}
	sum = 0
	for cur := dummy; cur != nil; cur = cur.next {
		sum += cur.val
		cur.next = last[sum].next
	}
	return dummy.next
}

func build(vals []int) *Node {
	var head *Node
	for i := len(vals) - 1; i >= 0; i-- {
		head = &Node{vals[i], head}
	}
	return head
}

func main() {
	head := removeZeroSum(build([]int{3, 4, -7, 5, -6, 6}))
	var parts []string
	for c := head; c != nil; c = c.next {
		parts = append(parts, strconv.Itoa(c.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
