// Day 1349: Reconstruct a BST from its postorder traversal.
// Consume postorder right-to-left with value bounds (right subtree before left). O(n) time, O(h) space.
package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Node struct {
	val   int
	left  *Node
	right *Node
}

var idx int
var post []int

func build(bound int) *Node {
	if idx < 0 || post[idx] < bound {
		return nil
	}
	node := &Node{val: post[idx]}
	idx--
	node.right = build(node.val)
	node.left = build(bound)
	return node
}

func preorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	*out = append(*out, n.val)
	preorder(n.left, out)
	preorder(n.right, out)
}

func inorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	inorder(n.left, out)
	*out = append(*out, n.val)
	inorder(n.right, out)
}

func join(xs []int) string {
	s := make([]string, len(xs))
	for i, x := range xs {
		s[i] = strconv.Itoa(x)
	}
	return strings.Join(s, " ")
}

func main() {
	post = []int{2, 4, 3, 8, 7, 5}
	idx = len(post) - 1
	root := build(math.MinInt32)
	var pre, in []int
	preorder(root, &pre)
	inorder(root, &in)
	fmt.Println("Root:", root.val)
	fmt.Println("Preorder:", join(pre))
	fmt.Println("Inorder:", join(in))
}
