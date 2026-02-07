// Day 1036: Reconstruct BST from postorder traversal.
// Approach: walk postorder in reverse (root,right,left) using value bounds.
// Time: O(n), Space: O(h) recursion.
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
		node := &Node{val: post[idx]}
		idx--
		node.right = build(node.val)
		node.left = build(bound)
		return node
	}
	return build(math.MinInt)
}

func printSideways(n *Node, depth int) {
	if n == nil {
		return
	}
	printSideways(n.right, depth+1)
	fmt.Println(strings.Repeat(" ", depth*4) + fmt.Sprint(n.val))
	printSideways(n.left, depth+1)
}

func main() {
	post := []int{2, 4, 3, 8, 7, 5}
	root := reconstruct(post)
	fmt.Println("Reconstructed BST (rotated 90 deg, root=5):")
	printSideways(root, 0)
}
