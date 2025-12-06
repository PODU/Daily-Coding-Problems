// Day 702: Serialize/deserialize a binary tree.
// Approach: preorder traversal with '#' null markers, comma-separated tokens.
// Both directions O(n) time and space.
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
	var sb strings.Builder
	var go_ func(n *Node)
	go_ = func(n *Node) {
		if n == nil {
			sb.WriteString("#,")
			return
		}
		sb.WriteString(n.val + ",")
		go_(n.left)
		go_(n.right)
	}
	go_(root)
	return sb.String()
}

func deserialize(s string) *Node {
	toks := strings.Split(strings.TrimSuffix(s, ","), ",")
	i := 0
	var go_ func() *Node
	go_ = func() *Node {
		t := toks[i]
		i++
		if t == "#" {
			return nil
		}
		n := &Node{val: t}
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
	back := deserialize(serialize(node))
	fmt.Println(back.left.left.val) // left.left
}
