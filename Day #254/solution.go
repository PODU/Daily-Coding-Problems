// Prune to full binary tree: post-order DFS; a node with exactly one child is
// replaced by that child. Returns new root. Time O(n), Space O(h) recursion.
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

func prune(root *Node) *Node {
	if root == nil {
		return nil
	}
	root.left = prune(root.left)
	root.right = prune(root.right)
	if root.left != nil && root.right == nil {
		return root.left
	}
	if root.right != nil && root.left == nil {
		return root.right
	}
	return root
}

func preorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	*out = append(*out, n.val)
	preorder(n.left, out)
	preorder(n.right, out)
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

	root = prune(root)
	var pre []int
	preorder(root, &pre)
	parts := make([]string, len(pre))
	for i, v := range pre {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("Preorder after pruning: " + strings.Join(parts, " "))
}
