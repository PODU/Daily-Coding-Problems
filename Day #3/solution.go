// Serialize/deserialize a binary tree via preorder traversal with '#' null markers.
// Time: O(n) for both, Space: O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	Val         string
	Left, Right *Node
}

func serialize(root *Node) string {
	var out []string
	var rec func(n *Node)
	rec = func(n *Node) {
		if n == nil {
			out = append(out, "#")
			return
		}
		out = append(out, n.Val) // assumes values contain no ','
		rec(n.Left)
		rec(n.Right)
	}
	rec(root)
	return strings.Join(out, ",")
}

func deserialize(s string) *Node {
	toks := strings.Split(s, ",")
	i := 0
	var rec func() *Node
	rec = func() *Node {
		t := toks[i]
		i++
		if t == "#" {
			return nil
		}
		n := &Node{Val: t}
		n.Left = rec()
		n.Right = rec()
		return n
	}
	return rec()
}

func main() {
	node := &Node{"root", &Node{"left", &Node{"left.left", nil, nil}, nil}, &Node{"right", nil, nil}}
	d := deserialize(serialize(node))
	fmt.Println(d.Left.Left.Val)
}
