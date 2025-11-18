// Deepest node in a binary tree via BFS level order; last visited node is deepest.
// Time O(N), Space O(N).
package main

import "fmt"

type Node struct {
	val         byte
	left, right *Node
}

func deepest(root *Node) byte {
	if root == nil {
		return 0
	}
	queue := []*Node{root}
	var last *Node
	for len(queue) > 0 {
		last = queue[0]
		queue = queue[1:]
		if last.left != nil {
			queue = append(queue, last.left)
		}
		if last.right != nil {
			queue = append(queue, last.right)
		}
	}
	return last.val
}

func main() {
	a := &Node{val: 'a'}
	b := &Node{val: 'b'}
	c := &Node{val: 'c'}
	d := &Node{val: 'd'}
	a.left, a.right = b, c
	b.left = d
	fmt.Printf("%c\n", deepest(a)) // d
}
