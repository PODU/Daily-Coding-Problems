// Day 1846: Serialize/deserialize a binary tree via preorder traversal with null markers.
// Time O(N), Space O(N). Escapes commas/backslashes so values round-trip.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         string
	left, right *Node
}

func ser(n *Node, sb *strings.Builder) {
	if n == nil {
		sb.WriteString("#,")
		return
	}
	for _, c := range n.val {
		if c == '\\' || c == ',' {
			sb.WriteByte('\\')
		}
		sb.WriteRune(c)
	}
	sb.WriteByte(',')
	ser(n.left, sb)
	ser(n.right, sb)
}

func serialize(root *Node) string {
	var sb strings.Builder
	ser(root, &sb)
	return sb.String()
}

func deser(s string, pos *int) *Node {
	var tok strings.Builder
	isNull := s[*pos] == '#'
	for *pos < len(s) && s[*pos] != ',' {
		if s[*pos] == '\\' {
			*pos++
			if *pos < len(s) {
				tok.WriteByte(s[*pos])
				*pos++
			}
		} else {
			tok.WriteByte(s[*pos])
			*pos++
		}
	}
	*pos++ // skip comma
	if isNull && tok.String() == "#" {
		return nil
	}
	n := &Node{val: tok.String()}
	n.left = deser(s, pos)
	n.right = deser(s, pos)
	return n
}

func deserialize(s string) *Node {
	pos := 0
	return deser(s, &pos)
}

func main() {
	node := &Node{"root",
		&Node{"left", &Node{"left.left", nil, nil}, nil},
		&Node{"right", nil, nil}}
	round := deserialize(serialize(node))
	fmt.Println(round.left.left.val == "left.left")
}
