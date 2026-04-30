// Day 1444: Convert a binary tree to a full binary tree by removing every node
// with exactly one child (its single child is promoted up).
// Post-order recursion. Time O(n), Space O(h).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func toFull(root *Node) *Node {
	if root == nil {
		return nil
	}
	root.left = toFull(root.left)
	root.right = toFull(root.right)
	if root.left != nil && root.right == nil {
		return root.left
	}
	if root.right != nil && root.left == nil {
		return root.right
	}
	return root
}

func preorder(r *Node, out *[]string) {
	if r == nil {
		return
	}
	*out = append(*out, fmt.Sprintf("%d", r.val))
	preorder(r.left, out)
	preorder(r.right, out)
}

func main() {
	n0 := &Node{val: 0, left: &Node{val: 1}, right: &Node{val: 2}}
	n0.left.left = &Node{val: 3}
	n0.left.left.right = &Node{val: 5}
	n0.right.right = &Node{val: 4}
	n0.right.right.left = &Node{val: 6}
	n0.right.right.right = &Node{val: 7}

	full := toFull(n0)
	var out []string
	preorder(full, &out)
	fmt.Println("Preorder of full tree: " + strings.Join(out, " ")) // 0 5 4 6 7
}
