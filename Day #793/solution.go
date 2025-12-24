// Prune binary tree to a full binary tree: post-order recursion; if a node has
// exactly one child, replace it with its (already pruned) child. O(n) time, O(h) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val   int
	left  *Node
	right *Node
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

func main() {
	n := make([]*Node, 8)
	for i := range n {
		n[i] = &Node{val: i}
	}
	n[0].left, n[0].right = n[1], n[2]
	n[1].left = n[3]
	n[2].right = n[4]
	n[3].right = n[5]
	n[4].left, n[4].right = n[6], n[7]

	root := prune(n[0])

	q := []*Node{root}
	for len(q) > 0 {
		var level []string
		var next []*Node
		for _, cur := range q {
			level = append(level, strconv.Itoa(cur.val))
			if cur.left != nil {
				next = append(next, cur.left)
			}
			if cur.right != nil {
				next = append(next, cur.right)
			}
		}
		fmt.Println(strings.Join(level, " "))
		q = next
	}
}
