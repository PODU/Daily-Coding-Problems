// Day 435: Reconstruct a binary tree from preorder + inorder traversals.
// Approach: recurse, using a map of inorder value->index to find roots in O(1).
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         string
	left, right *Node
}

func main() {
	preorder := []string{"a", "b", "d", "e", "c", "f", "g"}
	inorder := []string{"d", "b", "e", "a", "f", "c", "g"}

	idx := make(map[string]int)
	for i, v := range inorder {
		idx[v] = i
	}
	pi := 0
	var build func(inL, inR int) *Node
	build = func(inL, inR int) *Node {
		if inL > inR {
			return nil
		}
		rootVal := preorder[pi]
		pi++
		root := &Node{val: rootVal}
		mid := idx[rootVal]
		root.left = build(inL, mid-1)
		root.right = build(mid+1, inR)
		return root
	}
	root := build(0, len(inorder)-1)

	// Print level-order: a b c d e f g
	var out []string
	queue := []*Node{root}
	for len(queue) > 0 {
		n := queue[0]
		queue = queue[1:]
		out = append(out, n.val)
		if n.left != nil {
			queue = append(queue, n.left)
		}
		if n.right != nil {
			queue = append(queue, n.right)
		}
	}
	fmt.Println(strings.Join(out, " "))
}
