// Remove all consecutive nodes summing to zero using prefix sums + hash map.
// A prefix sum seen before means the span between is zero-sum and is excised.
// Time O(n), Space O(n).
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
	sum := 0
	for p := dummy; p != nil; p = p.next {
		sum += p.val
		seen[sum] = p
	}
	sum = 0
	for p := dummy; p != nil; p = p.next {
		sum += p.val
		p.next = seen[sum].next
	}
	return dummy.next
}

func main() {
	vals := []int{3, 4, -7, 5, -6, 6}
	var head *Node
	var tail *Node
	for _, v := range vals {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	head = removeZeroSum(head)
	var out []string
	for p := head; p != nil; p = p.next {
		out = append(out, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(out, " "))
}
