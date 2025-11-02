// Reconstruct BST from postorder: scan right-to-left (preorder of root,right,left)
// with an upper-bound recursion. Time O(n), space O(n).
package main

import (
	"fmt"
	"math"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func reconstruct(post []int) *Node {
	idx := len(post) - 1
	var build func(bound int) *Node
	build = func(bound int) *Node {
		if idx < 0 || post[idx] < bound {
			return nil
		}
		root := &Node{val: post[idx]}
		idx--
		root.right = build(root.val)
		root.left = build(bound)
		return root
	}
	return build(math.MinInt32)
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
	parts := make([]string, len(xs))
	for i, x := range xs {
		parts[i] = fmt.Sprintf("%d", x)
	}
	return strings.Join(parts, " ")
}

func main() {
	post := []int{2, 4, 3, 8, 7, 5}
	root := reconstruct(post)
	var pre, ino []int
	preorder(root, &pre)
	inorder(root, &ino)
	fmt.Println("preorder: " + join(pre))
	fmt.Println("inorder:  " + join(ino))
}
