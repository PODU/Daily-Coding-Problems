// Shuffle linked list uniformly via Fisher-Yates on node values.
// O(n) time, O(1) extra (in-place value swaps). Fixed seed -> deterministic.
package main

import (
	"fmt"
	"math/rand"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func main() {
	var head, tail *Node
	for v := 1; v <= 5; v++ {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}

	var nodes []*Node
	for p := head; p != nil; p = p.next {
		nodes = append(nodes, p)
	}

	rng := rand.New(rand.NewSource(42))
	for i := len(nodes) - 1; i > 0; i-- {
		j := rng.Intn(i + 1)
		nodes[i].val, nodes[j].val = nodes[j].val, nodes[i].val
	}

	var out []string
	for p := head; p != nil; p = p.next {
		out = append(out, fmt.Sprintf("%d", p.val))
	}
	fmt.Println(strings.Join(out, " "))
}
