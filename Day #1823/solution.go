// Convert to full binary tree by removing single-child nodes (post-order recursion).
// O(N) time, O(H) space.
package main

import (
	"fmt"
	"strconv"
)

type Node struct {
	val  int
	l, r *Node
}

func fullify(n *Node) *Node {
	if n == nil {
		return nil
	}
	n.l = fullify(n.l)
	n.r = fullify(n.r)
	if n.l == nil && n.r != nil {
		return n.r
	}
	if n.l != nil && n.r == nil {
		return n.l
	}
	return n
}

func serialize(n *Node) string {
	if n == nil {
		return ""
	}
	if n.l == nil && n.r == nil {
		return strconv.Itoa(n.val)
	}
	return strconv.Itoa(n.val) + "(" + serialize(n.l) + ", " + serialize(n.r) + ")"
}

func main() {
	nodes := make([]*Node, 8)
	for i := range nodes {
		nodes[i] = &Node{val: i}
	}
	nodes[0].l, nodes[0].r = nodes[1], nodes[2]
	nodes[1].l = nodes[3]
	nodes[2].r = nodes[4]
	nodes[3].r = nodes[5]
	nodes[4].l, nodes[4].r = nodes[6], nodes[7]
	root := fullify(nodes[0])
	fmt.Println(serialize(root)) // 0(5, 4(6, 7))
}
