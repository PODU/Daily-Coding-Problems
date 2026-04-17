// Uniform linked-list shuffle via Fisher-Yates. Time O(n), Space O(n) for the
// index pass (space-over-time variant: O(1) space, O(n^2) by random node selection).
// A deterministic LCG is used so output is reproducible.
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

var seed uint64 = 42

func nextRand() uint64 {
	seed = (seed*1103515245 + 12345) % 2147483648
	return seed
}

func shuffle(head *Node) *Node {
	var nodes []*Node
	for p := head; p != nil; p = p.next {
		nodes = append(nodes, p)
	}
	for i := len(nodes) - 1; i >= 1; i-- {
		j := int(nextRand() % uint64(i+1))
		nodes[i].val, nodes[j].val = nodes[j].val, nodes[i].val
	}
	return head
}

func main() {
	var head, tail *Node
	for v := 1; v <= 5; v++ {
		node := &Node{val: v}
		if head == nil {
			head, tail = node, node
		} else {
			tail.next = node
			tail = node
		}
	}
	head = shuffle(head)
	var out []string
	for p := head; p != nil; p = p.next {
		out = append(out, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(out, " -> "))
}
