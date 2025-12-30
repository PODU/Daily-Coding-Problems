// Merge two binary trees recursively; node value = sum, missing nodes taken from the other.
// Time: O(n), Space: O(h) recursion.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	Val         int
	Left, Right *Node
}

func merge(a, b *Node) *Node {
	if a == nil {
		return b
	}
	if b == nil {
		return a
	}
	n := &Node{Val: a.Val + b.Val}
	n.Left = merge(a.Left, b.Left)
	n.Right = merge(a.Right, b.Right)
	return n
}

func preorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	*out = append(*out, n.Val)
	preorder(n.Left, out)
	preorder(n.Right, out)
}

func main() {
	t1 := &Node{Val: 1, Left: &Node{Val: 3, Left: &Node{Val: 5}}, Right: &Node{Val: 2}}
	t2 := &Node{Val: 2,
		Left:  &Node{Val: 1, Right: &Node{Val: 4}},
		Right: &Node{Val: 3, Right: &Node{Val: 7}}}
	m := merge(t1, t2)
	var out []int
	preorder(m, &out)
	parts := make([]string, len(out))
	for i, v := range out {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
