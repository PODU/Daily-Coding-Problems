// Sorted array -> height-balanced BST: pick lower-mid as root, recurse halves; print BFS level-order.
// Time O(n), Space O(log n) recursion.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val  int
	l, r *Node
}

func build(a []int, lo, hi int) *Node {
	if lo > hi {
		return nil
	}
	mid := (lo + hi) / 2
	n := &Node{val: a[mid]}
	n.l = build(a, lo, mid-1)
	n.r = build(a, mid+1, hi)
	return n
}

func main() {
	a := []int{-10, -3, 0, 5, 9}
	root := build(a, 0, len(a)-1)
	var order []int
	q := []*Node{root}
	for len(q) > 0 {
		n := q[0]
		q = q[1:]
		if n == nil {
			continue
		}
		order = append(order, n.val)
		q = append(q, n.l, n.r)
	}
	parts := make([]string, len(order))
	for i, x := range order {
		parts[i] = strconv.Itoa(x)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
