// Day 94: Max path sum in a binary tree. DFS returns best downward gain; at each
// node consider a path bending through it. O(n) time, O(h) space.
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val         int
	left, right *Node
}

var best = math.MinInt

func gain(n *Node) int {
	if n == nil {
		return 0
	}
	l := max(gain(n.left), 0)
	r := max(gain(n.right), 0)
	best = max(best, n.val+l+r)
	return n.val + max(l, r)
}

func main() {
	root := &Node{val: -10,
		left:  &Node{val: 9},
		right: &Node{val: 20, left: &Node{val: 15}, right: &Node{val: 7}}}
	gain(root)
	fmt.Println(best) // 42
}
