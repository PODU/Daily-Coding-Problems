// Count nodes in a complete binary tree.
// Compare left/right spine heights: full subtree -> 2^h-1, else recurse.
// Time: O(log^2 n), Space: O(log n).
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
		return (1 << uint(lh)) - 1
	}
	return 1 + countNodes(root.left) + countNodes(root.right)
}

func main() {
	root := &Node{1,
		&Node{2, &Node{4, nil, nil}, &Node{5, nil, nil}},
		&Node{3, &Node{6, nil, nil}, nil}}
	fmt.Println(countNodes(root)) // 6
}
