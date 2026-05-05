// Day 1478: Return a deepest node of a binary tree.
// Single DFS returning (depth, node); keep the deeper subtree's result.
// Time O(N), Space O(H).
package main

import "fmt"

type Node struct {
	val         byte
	left, right *Node
}

func dfs(node *Node) (int, *Node) {
	if node == nil {
		return 0, nil
	}
	ld, ln := dfs(node.left)
	rd, rn := dfs(node.right)
	if ld >= rd {
		if node.left != nil {
			return ld + 1, ln
		}
		return ld + 1, node
	}
	return rd + 1, rn
}

func main() {
	root := &Node{'a',
		&Node{'b', &Node{'d', nil, nil}, nil},
		&Node{'c', nil, nil}}
	_, n := dfs(root)
	fmt.Printf("%c\n", n.val) // d
}
