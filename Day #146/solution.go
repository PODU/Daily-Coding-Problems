// Prune subtrees that contain only 0s via post-order recursion. O(n) time, O(h) stack.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func prune(root *Node) *Node {
	if root == nil {
		return nil
	}
	root.left = prune(root.left)
	root.right = prune(root.right)
	if root.val == 0 && root.left == nil && root.right == nil {
		return nil
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
	root := &Node{val: 0,
		left: &Node{val: 1},
		right: &Node{val: 0,
			left:  &Node{val: 1, left: &Node{val: 0}, right: &Node{val: 0}},
			right: &Node{val: 0}}}
	root = prune(root)
	var out []string
	preorder(root, &out)
	fmt.Println("preorder:", strings.Join(out, " ")) // 0 1 0 1
}
