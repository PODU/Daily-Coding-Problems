// Day 1495: Build a min-heap-ordered Cartesian tree whose in-order traversal is S.
// Approach: monotonic stack; pop nodes > current as its left subtree. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func buildCartesian(s []int) *Node {
	var stack []*Node
	for _, x := range s {
		cur := &Node{val: x}
		var last *Node
		for len(stack) > 0 && stack[len(stack)-1].val > x {
			last = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		cur.left = last
		if len(stack) > 0 {
			stack[len(stack)-1].right = cur
		}
		stack = append(stack, cur)
	}
	if len(stack) == 0 {
		return nil
	}
	return stack[0]
}

func inorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	inorder(n.left, out)
	*out = append(*out, n.val)
	inorder(n.right, out)
}

func pretty(n *Node, depth int) {
	if n == nil {
		return
	}
	pretty(n.right, depth+1)
	fmt.Println(strings.Repeat(" ", depth*4) + fmt.Sprint(n.val))
	pretty(n.left, depth+1)
}

func listing(n *Node) {
	if n == nil {
		return
	}
	if n.left != nil || n.right != nil {
		var kids []string
		if n.left != nil {
			kids = append(kids, fmt.Sprint(n.left.val))
		}
		if n.right != nil {
			kids = append(kids, fmt.Sprint(n.right.val))
		}
		fmt.Printf("  %d -> %s\n", n.val, strings.Join(kids, " "))
	}
	listing(n.left)
	listing(n.right)
}

func main() {
	s := []int{3, 2, 6, 1, 9}
	root := buildCartesian(s)

	var io []int
	inorder(root, &io)
	parts := make([]string, len(io))
	for i, v := range io {
		parts[i] = fmt.Sprint(v)
	}
	fmt.Println("In-order traversal: " + strings.Join(parts, " "))

	fmt.Printf("Root: %d\n", root.val)
	fmt.Println("Parent -> children:")
	listing(root)

	fmt.Println("Tree (rotated 90 deg, root at left):")
	pretty(root, 0)
}
