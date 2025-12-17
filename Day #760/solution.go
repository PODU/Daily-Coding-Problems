// Day 760: Uniformly shuffle a linked list. Space-prioritized variant:
// forward Fisher-Yates that swaps node values in place using O(1) extra space
// at the cost of O(n^2) time (re-walks to pick a uniform remaining node).
// A deterministic LCG is used so the demo output is reproducible.
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

type LCG struct{ s uint64 }

func (r *LCG) next() uint64 {
	r.s = (r.s*1103515245 + 12345) & 0x7fffffff
	return r.s
}

func shuffle(head *Node, rng *LCG) {
	for p := head; p != nil; p = p.next {
		m := 0
		for t := p; t != nil; t = t.next {
			m++
		}
		r := int(rng.next() % uint64(m))
		q := p
		for ; r > 0; r-- {
			q = q.next
		}
		p.val, q.val = q.val, p.val
	}
}

func printList(head *Node) {
	parts := []string{}
	for p := head; p != nil; p = p.next {
		parts = append(parts, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}

func main() {
	head := &Node{val: 1}
	cur := head
	for v := 2; v <= 5; v++ {
		cur.next = &Node{val: v}
		cur = cur.next
	}

	fmt.Print("original: ")
	printList(head)
	shuffle(head, &LCG{s: 42})
	fmt.Print("shuffled: ")
	printList(head)
}
