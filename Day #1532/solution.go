// Invert (mirror) a binary tree by swapping left/right children of every node.
// Time O(n), Space O(h) recursion.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  byte
	l, r *Node
}

func invert(root *Node) *Node {
	if root == nil {
		return nil
	}
	root.l, root.r = root.r, root.l
	invert(root.l)
	invert(root.r)
	return root
}

func preorder(root *Node, out *[]string) {
	if root == nil {
		return
	}
	*out = append(*out, string(root.val))
	preorder(root.l, out)
	preorder(root.r, out)
}

func main() {
	a := &Node{val: 'a',
		l: &Node{val: 'b', l: &Node{val: 'd'}, r: &Node{val: 'e'}},
		r: &Node{val: 'c', l: &Node{val: 'f'}}}
	var before, after []string
	preorder(a, &before)
	invert(a)
	preorder(a, &after)
	fmt.Println("before (preorder):", strings.Join(before, " "))
	fmt.Println("after  (preorder):", strings.Join(after, " "))
}
