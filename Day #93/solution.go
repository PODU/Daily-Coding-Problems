// Day 93: Largest BST subtree size. Post-order DFS returning (isBST, size, min,
// max) per node and combining children. O(n) time, O(h) space.
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val         int
	left, right *Node
}

type info struct {
	bst      bool
	size     int
	mn, mx   int
}

var best = 0

func dfs(n *Node) info {
	if n == nil {
		return info{true, 0, math.MaxInt, math.MinInt}
	}
	l, r := dfs(n.left), dfs(n.right)
	if l.bst && r.bst && l.mx < n.val && n.val < r.mn {
		sz := l.size + r.size + 1
		if sz > best {
			best = sz
		}
		return info{true, sz, min(l.mn, n.val), max(r.mx, n.val)}
	}
	return info{false, 0, 0, 0}
}

func main() {
	root := &Node{val: 10,
		left:  &Node{val: 5, left: &Node{val: 1}, right: &Node{val: 8}},
		right: &Node{val: 15, right: &Node{val: 7}}}
	dfs(root)
	fmt.Println(best) // 3
}
