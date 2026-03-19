// Serialize/deserialize a binary tree via preorder with null markers.
// Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	Val         string
	Left, Right *Node
}

func ser(n *Node, sb *strings.Builder) {
	if n == nil {
		sb.WriteString("#|")
		return
	}
	for _, c := range n.Val {
		if c == '\\' || c == '|' {
			sb.WriteByte('\\')
		}
		sb.WriteRune(c)
	}
	sb.WriteByte('|')
	ser(n.Left, sb)
	ser(n.Right, sb)
}

func serialize(root *Node) string {
	var sb strings.Builder
	ser(root, &sb)
	return sb.String()
}

func deserialize(s string) *Node {
	var toks []string
	var cur strings.Builder
	for i := 0; i < len(s); i++ {
		if s[i] == '\\' {
			i++
			cur.WriteByte(s[i])
		} else if s[i] == '|' {
			toks = append(toks, cur.String())
			cur.Reset()
		} else {
			cur.WriteByte(s[i])
		}
	}
	pos := 0
	var build func() *Node
	build = func() *Node {
		v := toks[pos]
		pos++
		if v == "#" {
			return nil
		}
		n := &Node{Val: v}
		n.Left = build()
		n.Right = build()
		return n
	}
	return build()
}

func main() {
	node := &Node{Val: "root",
		Left:  &Node{Val: "left", Left: &Node{Val: "left.left"}},
		Right: &Node{Val: "right"}}
	d := deserialize(serialize(node))
	fmt.Println(d.Left.Left.Val)
}
