// Day 1755: Count nodes in a COMPLETE binary tree in better than O(n).
// Compare left/right spine heights: if equal, subtree is perfect (2^h - 1);
// else 1 + recurse on both children. Time O(log^2 n), Space O(log n).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func leftHeight(n *Node) int {
	h := 0
	for n != nil {
		h++
		n = n.left
	}
	return h
}

func rightHeight(n *Node) int {
	h := 0
	for n != nil {
		h++
		n = n.right
	}
	return h
}

func countNodes(root *Node) int {
	if root == nil {
		return 0
	}
	lh, rh := leftHeight(root), rightHeight(root)
	if lh == rh {
		return (1 << uint(lh)) - 1 // perfect subtree
	}
	return 1 + countNodes(root.left) + countNodes(root.right)
}

func main() {
	// Complete binary tree with 6 nodes (values 1..6):
	//          1
	//        /   \
	//       2     3
	//      / \   /
	//     4   5 6
	root := &Node{val: 1}
	root.left = &Node{val: 2}
	root.right = &Node{val: 3}
	root.left.left = &Node{val: 4}
	root.left.right = &Node{val: 5}
	root.right.left = &Node{val: 6}

	fmt.Println(countNodes(root)) // expected: 6
}
