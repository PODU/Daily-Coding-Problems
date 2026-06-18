// BFS level order; last node dequeued is a deepest node. Time O(n), Space O(n).
package main

import "fmt"

type Node struct {
	val   byte
	left  *Node
	right *Node
}

func deepestNode(root *Node) *Node {
	if root == nil {
		return nil
	}
	queue := []*Node{root}
	var last *Node = root
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
	return last
}

func main() {
	a := &Node{val: 'a'}
	b := &Node{val: 'b'}
	c := &Node{val: 'c'}
	d := &Node{val: 'd'}
	a.left, a.right = b, c
	b.left = d
	fmt.Printf("%c\n", deepestNode(a).val)
}
