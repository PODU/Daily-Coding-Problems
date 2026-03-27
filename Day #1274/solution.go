// Day 1274: Prune a 0/1 binary tree, removing every subtree that contains only 0s.
// Post-order recursion: a node survives iff it is 1 or has a surviving child. O(n).
package main

import (
	"fmt"
	"strconv"
)

type Node struct {
	Val         int
	Left, Right *Node
}

func prune(node *Node) *Node {
	if node == nil {
		return nil
	}
	node.Left = prune(node.Left)
	node.Right = prune(node.Right)
	if node.Val == 0 && node.Left == nil && node.Right == nil {
		return nil
	}
	return node
}

func serialize(node *Node) string {
	if node == nil {
		return "null"
	}
	return strconv.Itoa(node.Val) + "(" + serialize(node.Left) + "," + serialize(node.Right) + ")"
}

func main() {
	root := &Node{Val: 0,
		Left: &Node{Val: 1},
		Right: &Node{Val: 0,
			Left:  &Node{Val: 1, Left: &Node{Val: 0}, Right: &Node{Val: 0}},
			Right: &Node{Val: 0},
		},
	}
	root = prune(root)
	// Pruned tree: 0(1(null,null),0(1(null,null),null))
	fmt.Println(serialize(root))
}
