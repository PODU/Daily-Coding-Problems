// Day 997: Serialize / deserialize a binary tree.
// Preorder traversal with "#" markers for null children, comma-joined.
// Both serialize and deserialize are O(n) time and space.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         string
	left, right *Node
}

func serialize(root *Node) string {
	var out []string
	var go_ func(n *Node)
	go_ = func(n *Node) {
		if n == nil {
			out = append(out, "#")
			return
		}
		out = append(out, n.val)
		go_(n.left)
		go_(n.right)
	}
	go_(root)
	return strings.Join(out, ",")
}

func deserialize(s string) *Node {
	toks := strings.Split(s, ",")
	i := 0
	var go_ func() *Node
	go_ = func() *Node {
		v := toks[i]
		i++
		if v == "#" {
			return nil
		}
		n := &Node{val: v}
		n.left = go_()
		n.right = go_()
		return n
	}
	return go_()
}

func main() {
	node := &Node{val: "root",
		left:  &Node{val: "left", left: &Node{val: "left.left"}},
		right: &Node{val: "right"}}
	s := serialize(node)
	fmt.Println(s)
	fmt.Println("assertion passed:", deserialize(s).left.left.val) // left.left
}
