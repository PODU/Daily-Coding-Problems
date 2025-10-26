// Convert binary tree to FULL binary tree by removing nodes with exactly one child.
// Post-order recursion: a one-child node is replaced by its processed child.
// Time: O(n), Space: O(h) recursion.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func fullTree(root *Node) *Node {
	if root == nil {
		return nil
	}
	root.left = fullTree(root.left)
	root.right = fullTree(root.right)
	if root.left != nil && root.right == nil {
		return root.left
	}
	if root.right != nil && root.left == nil {
		return root.right
	}
	return root
}

func preorder(root *Node, out *[]string) {
	if root == nil {
		return
	}
	*out = append(*out, strconv.Itoa(root.val))
	preorder(root.left, out)
	preorder(root.right, out)
}

func main() {
	root := &Node{val: 0}
	root.left = &Node{val: 1}
	root.right = &Node{val: 2}
	root.left.left = &Node{val: 3}
	root.left.left.right = &Node{val: 5}
	root.right.right = &Node{val: 4}
	root.right.right.left = &Node{val: 6}
	root.right.right.right = &Node{val: 7}

	root = fullTree(root)
	var out []string
	preorder(root, &out)
	fmt.Println(strings.Join(out, " "))
}
