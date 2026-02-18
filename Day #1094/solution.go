// Uniformly shuffle a linked list via Fisher-Yates on node values. Time O(n), Space O(n).
// Space-over-time alternative: walk to a random remaining node and swap in place -> O(1) extra, O(n^2) time.
package main

import (
	"fmt"
	"math/rand"
)

type Node struct {
	val  int
	next *Node
}

func build(arr []int) *Node {
	var head, tail *Node
	for _, x := range arr {
		n := &Node{val: x}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	return head
}

func toSlice(h *Node) []int {
	var a []int
	for c := h; c != nil; c = c.next {
		a = append(a, c.val)
	}
	return a
}

func shuffleList(head *Node, r *rand.Rand) {
	var nodes []*Node
	for c := head; c != nil; c = c.next {
		nodes = append(nodes, c)
	}
	for i := len(nodes) - 1; i > 0; i-- {
		j := r.Intn(i + 1)
		nodes[i].val, nodes[j].val = nodes[j].val, nodes[i].val
	}
}

func main() {
	head := build([]int{1, 2, 3, 4, 5})
	fmt.Println("Original:", toSlice(head))
	r := rand.New(rand.NewSource(42))
	shuffleList(head, r)
	fmt.Println("Shuffled:", toSlice(head))
}
